# The `tectonic_engine_spx2html` crate

[![](http://meritbadge.herokuapp.com/tectonic_engine_spx2html)](https://crates.io/crates/tectonic_engine_spx2html)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides and engine
that converts "SPX" (semantically paginated XDV) files, a customized Tectonic
output, to HTML.

- [API documentation](https://docs.rs/tectonic_engine_spx2html/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

HTML templates use Tera 2 syntax. Existing Tera 1 templates may need changes,
including replacing macros with components and renaming the `escape` filter to
`escape_html`; see the [Tera migration guide](https://github.com/Keats/tera/blob/master/MIGRATION.md).


## Cargo features

This crate currently provides no [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
