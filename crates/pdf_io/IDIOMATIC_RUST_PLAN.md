# Incremental plan for idiomatic Rust in pdf_io

## Goal and starting point

Replace the translated implementation in `src/backend/` with ordinary Rust
ownership, explicit document state, checked parsers, and typed errors. Preserve
PDF output, diagnostic behavior, performance, and engine integrations throughout.

The implementation is Rust, but retains raw pointers, manual allocation and
reference counting, mutable statics, duplicated C layouts, and internal calls
through `extern "C-unwind"` declarations linked to `__pdf_impl_*` symbols.
`compat/` exposes the C API and invokes native aborts after Rust returns.
Expected fatal errors currently use unwinding, requiring `panic = "unwind"`.

Build on the existing Rust modules in `src/`: `pipeline`, `compress`,
`predictor`, `fontmap`, `preflight`, and `tfm`. Preserve owned page plans, the
bounded worker budget, backpressure, and ordered result commitment.

## Rules for each change

- Migrate one ownership boundary or small dependency cluster per PR. Keep each
  intermediate revision buildable and usable by the existing engines.
- Implement a typed Rust operation, adapt the old entry point, then migrate
  callers. Delete the superseded implementation as its callers move; avoid
  maintaining two production backends indefinitely.
- Separate behavior fixes from structural changes. For a discovered defect,
  first add a regression and make the smallest fix.
- Keep names, strings, character maps, and font data as bytes unless their
  encoding is known. Do not assume UTF-8.
- Preserve dictionary traversal, object numbering, resource ordering, numeric
  formatting, and compression decisions. Container changes can alter PDF bytes.
- Keep unsafe conversions at documented compatibility boundaries. Do not solve
  ownership problems with widespread casts, `RefCell`, or lifetime extension.
- Introduce traits, arenas, shared ownership, or dependencies for an identified
  requirement. Do not replace every C allocation with a separate `Box`.
- Remove lint allowances as modules become ordinary Rust. Avoid a large
  naming-only or formatting-only pass over translated code.

## Validation gates

Retain a baseline revision and export its benchmark PDFs outside the checkout.
Record toolchain, profile, worker count, inputs, and reproducibility settings.
Temporary test-only comparison against a baseline is sufficient; a permanent
second backend is unnecessary.

Run focused tests during development and the workspace suite before merging:

```sh
cargo test -p tectonic_pdf_io --lib --locked
cargo test --test pdf_parallel --test tex-outputs --locked
cargo clippy -p tectonic_pdf_io --all-targets --locked
cargo test --workspace --locked
cargo test --release --test pdf_parallel --test tex-outputs --locked
```

Use `benchmark_single_pdf_compilation` in `tests/pdf_parallel.rs` to measure
text, native fonts, graphics, images, and attachments under legacy, one-worker,
and automatic-worker configurations. `TECTONIC_PDF_BENCH_DIR` exports PDFs;
the workload and backend filters allow focused measurements. Compare warm
medians and peak memory. Set a regression budget from measured variance and
investigate repeatable regressions before proceeding.

Default to exact output comparisons. For an intentional byte change, explain
it and independently check rendering, extracted text, resources, and
cross-reference integrity before changing expectations. Preserve diagnostics
relevant to the changed path.

Add cases for malformed input, partial initialization, and early returns when
changing ownership. Verify a successful build after failure in the same process.
Exercise backpressure and synchronous execution when changing workers. Use
fuzzing, Miri on pure Rust components, and sanitizers where appropriate.

Validate platform-facing changes on Linux, macOS, and Windows. Check endian,
alignment, and integer-width assumptions in binary formats and ABI types.

## Stage 1: Safe leaf utilities

Start with `dpx_unicode`, pure conversion helpers in `dpx_numbers`, and bounded
reads currently spread across `dpx_mfileio` and `dpx_sfnt`.

1. Extract Unicode operations into functions over slices and explicit code
   points. Return consumed lengths and typed decoding errors.
2. Introduce a small byte cursor for checked reads, skips, and big-endian
   integers. Retain offsets and distinguish truncation from invalid values.
3. Rewrite numeric output operations individually using caller-owned buffers.
   Preserve rounding, precision, negative zero, and exact emitted bytes.
4. Keep pointer/length conversion and legacy return values in adapters.

**Exit:** converted functions have no raw pointer traversal, mutable statics,
or C allocation. Boundary-value, invalid-encoding, truncation, and exact-output
tests pass. Translated callers remain functional through adapters.

Implemented in `src/unicode.rs`, `src/byte_cursor.rs`, `src/numbers.rs`,
`src/line.rs`, and `src/sfnt.rs`. Compatibility adapters retain Unicode's
historical acceptance rules and output bytes, numeric rounding and negative-zero
behavior, and line-ending/EOF handling. SFNT directory fields and checksums use
checked slices; directory ownership remains for later stages.

## Stage 2: Real Rust interfaces between modules

Begin with renderer types shared by `pipeline.rs` and `dpx_pdfdev`, then convert
imports one dependency cluster at a time.

1. Extract shared renderer types from the translated module. Distinguish owned
   Rust types from representations required by C.
2. Establish one definition for each ABI type as its cluster migrates. Verify
   size, alignment, field offsets, signedness, and ownership against C headers.
3. Replace internal symbol-name imports with normal module calls using those
   shared types or the new typed API.
4. Confine raw handles and exported symbols to a compatibility module. Audit
   engine consumers and published headers before deleting an entry point.

**Exit:** converted clusters use ordinary Rust calls and canonical types.
Internal Rust code does not call their C adapters. Existing C consumers retain
their calling conventions and pointer-lifetime guarantees.

Implemented for the renderer/geometry cluster in `src/renderer.rs` and
`src/compat/renderer.rs`. Page plans pass borrowed command bytes and copied
font snapshots through Rust calls. Geometry types have one Rust definition
across the translated modules; the C headers remain unchanged. Renderer and
geometry entry points stay in compatibility code, while their implementations
and translated callers use normal module calls. Unicode validity and fixed-word
scaling callers use the Stage 1 functions directly.

`compat/renderer_layout.c` and its Rust test compare sizes, alignments, every
field offset, primitive widths, and character signedness against the published
headers on the build target. Borrowed inputs remain live until synchronous
rendering returns; recorded text and fonts are copied into the owned page plan.
The remaining dependency clusters migrate incrementally in later changes.

## Stage 3: Typed errors and explicit I/O

Target `runtime.rs`, `macros.rs`, `dpx_dpxfile`, `dpx_mfileio`, and leaf parsers.

1. Introduce `PdfError` with distinctions concrete callers need: malformed
   input, unsupported data, I/O failure, and invalid document state. Include
   useful file/offset context without accidentally reordering diagnostics.
2. Return `Result` from migrated operations and propagate with `?`. Convert
   errors to legacy status/error storage only at the compatibility edge.
3. Pass I/O and diagnostic capabilities explicitly. Reuse `Read`, `Seek`, and
   `Write` where their semantics fit; avoid a speculative general I/O framework.
4. Replace Rust-side variadic logging with Rust formatting while preserving
   content, ordering, quietness, and the existing truncation policy.

When typed code calls translated code that still raises `runtime::Abort`, use
a transitional adapter to catch that failure and return `PdfError`. Do not
catch failures around every primitive operation.

**Exit:** expected errors in migrated paths use `Result`, and early returns
release owned values. No native `longjmp` crosses Rust. Keep outer panic
containment and the unwind requirement until all error paths and engine
boundaries have been audited; their removal is a separate decision.

Implemented for the migrated renderer and leaf I/O paths. `src/error.rs`
provides typed error kinds, byte-preserving diagnostics and filenames, offsets,
I/O operation context, and underlying I/O errors. Font-header probes, stdio
size/seek helpers, and SFNT directory reads accept explicit `Read`/`Seek`
inputs and return `Result`. Bridge and FILE adapters stay in `src/compat/input.rs`;
legacy header rejection, read diagnostics, and seek behavior are preserved.
Compatibility SFNT directory allocations are initialized before fallible reads,
so cleanup after partial initialization cannot traverse uninitialized pointers.

Worker channels retain typed failures through ordered result collection and
release. `runtime::catch_backend` translates expected failures from remaining
translated code at operation boundaries and rethrows unexpected panics; outer
compatibility and worker boundaries still contain them. Compatibility entry
points translate typed results to legacy status/error storage.

Migrated SFNT diagnostics use Rust formatting through `src/diagnostics.rs`.
The formatter retains the 1023-byte payload limit and embedded-NUL handling.
Emission accepts explicit bridge state while the compatibility wrapper retains
quietness and warning ordering. Printf support remains for untranslated callers;
document-global state, raw font-directory ownership, and further parser
migrations remain in their later stages.

## Stage 4: An owned page renderer

Target the worker-only portion of `dpx_pdfdev` and its pipeline interface.
Workers already receive owned plans and explicit device data, making this a
useful early milestone for text-heavy builds.

1. Separate worker rendering from coordinator font/resource lookup.
2. Use enums for commands and device modes. Accept slices and write into a
   borrowed output buffer, removing raw output callbacks for Rust callers.
3. Move positioning, escaping, and rule output to a `PageRenderer` with explicit
   state and typed errors.
4. Retain adjacent text batching and reusable buffers. Avoid per-glyph
   allocation and repeated font-data cloning.

**Exit:** worker rendering is safe Rust with no bridge or global-state access.
Page bytes match under every worker and queue configuration. Raw pointers
remain only in external adapters that still need them.

Implemented in `src/renderer.rs`. `PageRenderer` uses explicit command,
direction, rotation, and device-mode enums, borrowed font/command data, and a
borrowed output Vec. Positioning, escaping, rules and numeric output use safe
Rust. The pipeline retains adjacent text batching and its owned page buffers;
it renders directly into the output without allocating an ABI command array.
The former translated worker renderer and raw-output device branch are removed.
The C adapter retains its published layouts and symbols. Golden byte snapshots
cover font modes, transformations, escaping, precision and direction combinations.

## Stage 5: Document-owned state

Inventory mutable statics in DVI interpretation, device/document handling,
resources, fonts, font maps, and special handlers. Classify each as immutable
data, document state, or a justified shared cache with explicit invalidation.

1. Move configuration and diagnostics first, then device/page state, resource
   registries, font caches, and special-handler state in separate PRs.
2. Give each group an owner and pass focused borrows to consumers. Avoid a
   replacement context borrowed mutably by every operation.
3. Represent partial initialization and completion explicitly. Replace global
   cleanup/reset sequences with ownership and lifecycle operations.
4. If the C API temporarily requires a current-context binding, install it with
   a scoped guard, reject reentrancy, and keep it out of core code.

**Exit:** state is created and destroyed with its document. Failure and
consecutive builds cannot leak settings or resources. Retain the engine lock
until the bridge and native engines also support independent execution;
moving PDF statics alone does not establish thread safety.

Implemented with `compat::document::DocumentScope`, 29 focused translated state
groups (139 fields), document-owned Rust caches/diagnostics, and scoped allocation
ownership. Both native engine wrappers end their scope on success and abort.
Nesting/overlap is rejected, partial allocations are reclaimed, and caller-owned
C buffers retain their ownership when returned through adapters. Published ABI
data keeps its symbol addresses and is exclusively reserved/reset per scope.
See [the state inventory](DOCUMENT_STATE_INVENTORY.md) for ownership boundaries,
immutable tables, compatibility storage, shared pools and remaining constraints.
The native engine lock and translated pointer-based object representation remain.

Output comparisons, benchmark measurements and test results for both stages
are recorded in [the validation report](STAGES_4_5_VALIDATION.md).

## Stage 6: PDF object ownership

Target `dpx_pdfobj`, `dpx_pdfnames`, and `dpx_pdfresource`. Split storage,
container operations, and writer integration into separate PRs.

1. Replace tag-plus-void-pointer values with an enum of concrete PDF values.
   Introduce distinct types for object IDs, indirect references, and resources.
2. Define graph ownership before selecting storage. Preserve identity and
   support indirect-reference cycles without ownership cycles. A document-owned
   arena with typed IDs is one candidate, not an automatic requirement.
3. Replace manual array/dictionary growth with owned containers that preserve
   traversal order and replacement behavior.
4. Maintain stable compatibility representations while C retains pointers.
   Never expose movable `Vec` elements, reinterpret an old pointer as a new
   Rust enum, or recycle handles still visible to C.
5. Migrate writer/resource callers, then remove manual reference counting and
   C allocation from the converted graph.

**Exit:** identity, mutation, ownership, and release are explicit. Tests cover
shared references, indirect cycles, replacement, deferred worker results, and
failure cleanup. Object numbering and serialized bytes remain stable.

Implemented with concrete values and typed IDs in `src/objects.rs`, stable
document-owned slots in `src/compat/objects.rs`, ordered owned array/dictionary
containers, and owned name/resource registries. Writer and worker-result paths
use the new representations. Compatibility leases preserve release-triggered
serialization without owning graph allocations. Forward-reference identities
survive placeholder retirement. See [object ownership](OBJECT_OWNERSHIP.md) for
the C handle contract and the stream-buffer/writer boundaries retained for
Stages 8 and 9, and [validation](STAGE_6_VALIDATION.md) for checks and measurements.

## Stage 7: Checked parsers and owned font data

The Stage 1 cursor can be adopted early. Object-producing PDF parsing should
follow Stage 6.

1. Convert `dpx_pdfparse` and `dpx_pst_obj` tokenization to checked slices.
   Review nesting and length arithmetic; document deliberate changes to how
   malformed input is accepted or rejected.
2. Convert SFNT tables in `dpx_tt_table`, `dpx_tt_cmap`, and `dpx_tt_post` one
   family at a time, then glyph/subset handling and GSUB.
3. Convert CFF indexes/dictionaries before charsets, subroutines, and Type 2
   charstrings. Handle Type 1, PK, and virtual fonts separately.
4. Reuse backing font buffers and byte ranges. Keep borrowed views within their
   backing storage's lifetime. Share ownership only when independent workers
   actually need it.

**Exit:** converted parsers validate offsets, lengths, and arithmetic before
access. Subset bytes, glyph mappings, metrics, extracted text, and diagnostics
match. Delete each old parser as production callers migrate.

## Stage 8: Image and encryption ownership

This can proceed alongside font work after typed I/O and errors exist.

1. Convert BMP, JPEG, and JP2 headers to checked reads. Preserve density,
   profiles, orientation, transparency, and encoded-data pass-through.
2. Make raster and stream ownership explicit while retaining parallel
   predictors and the shared compression budget.
3. Contain libpng pointers, callbacks, and cleanup in a small audited wrapper.
   Replacing libpng is a separate compatibility/performance decision.
4. Evaluate reviewed Rust crypto implementations behind the current PDF policy.
   Preserve algorithms, padding, derivation, permissions, and deterministic
   fixtures. Keep security-policy and randomness changes separate.

**Exit:** failures release native/image resources and raster arithmetic is
checked. Image fixtures and encryption vectors match. Cover padding/block
boundaries, key sizes, chunked hashing, and supported PDF encryption revisions.

## Stage 9: Explicit interpretation and writing

Target remaining DVI/VF execution, drawing, special handlers, document
finalization, and stream/cross-reference writing.

1. Use owned values for graphics state and interpreter stacks. Introduce typed
   IDs at lookup boundaries instead of casting integers throughout execution.
2. Convert special-handler families individually. Preserve source order,
   coordinator execution, and one-time side effects.
3. Make lifecycle transitions and finalization explicit. Finish worker jobs
   before final stream lengths and cross-reference offsets are committed.
4. Own pending output and check offset arithmetic. Preserve object streams,
   encryption integration, metadata, and deterministic commit order.

**Exit:** the coordinator owns document state; workers own independent inputs.
Finalization and failure paths are explicit. Unknown specials, virtual fonts,
imported PDFs, links, annotations, and outlines retain their behavior.

## Stage 10: Retire translated infrastructure

1. Delete superseded types, allocation helpers, duplicate bindings, unused
   implementation symbols, and broad lint suppressions.
2. Audit remaining unsafe blocks for a specific ABI, native-library, or measured
   low-level requirement. Add local safety explanations and use
   `#![forbid(unsafe_code)]` in safe modules where practical.
3. Preserve C APIs needed by XeTeX, the xdvipdfmx launcher, or supported external
   consumers. Remove adapters after consumers migrate or an explicit API
   compatibility decision is made.
4. Revisit panic containment, release panic policy, and engine-lock scope only
   after expected-error propagation and cross-engine lifetime audits pass.

**Done means:** core operations have typed APIs and explicit ownership;
document state is owned; parsers check bounds; expected failures return errors;
unsafe code is confined to justified boundaries; and output, recovery,
performance, and platform validation pass.

## Recommended first PRs

1. Extract safe byte-oriented Unicode operations from `dpx_unicode`, retaining
   existing C entry points as adapters.
2. Extract shared renderer types and replace the raw output callback for Rust
   callers with a borrowed output buffer.
3. Introduce the checked byte cursor and migrate one small SFNT table parser.
4. Introduce `PdfError` for these modules and adapt their existing entry points.
5. Move worker rendering into a safe `PageRenderer` implementation.

Then choose the next cluster from measured text-build costs and ownership
dependencies. Treat global state, fonts, and PDF objects as explicit milestones
rather than combining them into one rewrite.
