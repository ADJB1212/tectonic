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
The coordinator phases distinguish font-map loading, DVI preparation, document
setup, page interpretation, and finalization. Worker elapsed times overlap each
other and coordinator phases.

TeX source readers buffer 8 KiB at a time to amortize bridge dispatch and input
hashing. Rust loads font maps directly through the input provider, scans lines,
parses common DVIPS records, and indexes them in a hash map. Parsed fields refer
to a shared input buffer; stable C records and strings are allocated only when a
font is used. DVIPDFM options, subfont expansion, escaped strings, and verbose
diagnostics retain their C compatibility handlers. Those handlers run after
Rust returns, so native aborts never cross a Rust stack frame. Map changes remain
local to each build, and the runtime releases Rust storage after native aborts.

Rust also scans DVI pages for worker eligibility, reading one page at a time
directly through the input provider and reusing its buffer. It skips font
definitions and glyph runs without decoding or allocating their contents, and
shares the special classifier with the native entry point. The same decoder
builds playback buffers for both PDF backends: it validates complete pages,
removes font definitions, and records special-command spans. C receives a copy
of the compacted page and applies specials in source order after Rust returns.
Malformed input and pTeX direction commands return to the native scanner for
validation and state updates. Linear input retains its existing scanner.
The engine releases Rust input buffers after success or native abort, including
when the legacy backend is selected.

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
median times with automatic workers, before and after the Rust font-map and
DVI preflight ports. The C baseline already includes buffered source/map input
and a larger font-map hash table. Total time includes TeX layout and PDF generation.

| Workload | PDF before | PDF after | Total before | Total after |
| --- | ---: | ---: | ---: | ---: |
| 200 text pages | 43.98 ms | 32.97 ms | 86.22 ms | 76.52 ms |
| 24 native-font pages | 22.53 ms | 15.32 ms | 45.07 ms | 37.63 ms |
| 100 rule-heavy pages | 25.36 ms | 17.99 ms | 68.85 ms | 63.60 ms |
| 16 independent 1024x1024 images | 91.07 ms | 84.53 ms | 105.83 ms | 99.25 ms |
| 128 attachments | 35.08 ms | 28.23 ms | 50.31 ms | 45.99 ms |

The text workload improved about 25% in the PDF stage and 11% end to end.
The native-font workload improved about 32% in the PDF stage and 16% end to end.
Font-map loading fell from about 14 ms to 9 ms, and text DVI preparation from
5.45 ms to 0.70 ms. The font-map port also applies to the legacy backend.
All 15 exported PDFs were byte-identical before and after the changes. Results
depend on document content and font-map size.

Separate text-only processes measured peak RSS of 190 MB for the C baseline and
195 MB for the Rust ports, including fixtures and repeated builds. The lazy map
index retains source buffers until the build ends. Reduce workers or the queue
budget when memory is constrained.

The subsequent playback-buffer port was measured separately against the Rust
font-map/preflight implementation, using the same five-warm-run procedure.
For 200 text pages, PDF-stage medians were:

| Backend | Native playback scan | Rust playback scan |
| --- | ---: | ---: |
| Legacy | 36.41 ms | 33.47 ms |
| One worker | 38.69 ms | 37.94 ms |
| Automatic workers | 31.59 ms | 31.48 ms |

The legacy backend improved about 8%; automatic-worker timing was effectively
unchanged. All 15 workload/backend PDFs remained byte-identical. These figures
isolate page loading and scanning; text interpretation still runs in C.


## Cargo features

This crate currently provides no [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
