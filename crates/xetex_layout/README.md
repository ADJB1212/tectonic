# The `tectonic_xetex_layout` crate

[![](http://meritbadge.herokuapp.com/tectonic_xetex_layout)](https://crates.io/crates/tectonic_xetex_layout)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides the font
loading and layout routines of [XeTeX] as a crate, *currently providing only a C
API*.

[XeTeX]: http://xetex.sourceforge.net/

- [API documentation](https://docs.rs/tectonic_xetex_layout/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_TECTONIC_XETEX_LAYOUT_INCLUDE_PATH`, which
will be a semicolon-separated list of directories enabling your code to include
the `tectonic_xetex_layout.h` header.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_xetex_layout;
```


## Parallel paragraph shaping

Final paragraph lines are prepared before packing so that independent native
OpenType text runs can be shaped concurrently. Workers use separate HarfBuzz
buffers and immutable font configuration; FreeType access remains protected by
the font's mutex. TeX node changes, box packing, math, and page building stay on
the engine thread.

The parallel path currently handles printable ASCII runs containing letters,
with at least four runs and 256 UTF-16 units per paragraph. Other text, Graphite,
AAT, and small batches retain serial shaping. Prepared buffers are consumed
during line packing and unused buffers are released after the paragraph.

The persistent pool uses the available CPU count up to eight workers.
`TECTONIC_LAYOUT_THREADS` can lower this limit; set it to `1` to use serial
shaping. The setting is read when the pool is first needed in a process.

## Cargo features

This crate provides the following [Cargo features][features]:

[features]: https://doc.rust-lang.org/cargo/reference/features.html

- **`external-harfbuzz`**: activates the same-named feature in
  the `tectonic_bridge_harfbuzz` dependency.
