# The `tectonic_bundles` crate

[![](http://meritbadge.herokuapp.com/tectonic_bundles)](https://crates.io/crates/tectonic_bundles)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It implements various
Tectonic "bundles" that provide access to collections of TeX support files.

- [API documentation](https://docs.rs/tectonic_bundles/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).


## Cargo features

This crate provides the following [Cargo features][features]:

[features]: https://doc.rust-lang.org/cargo/reference/features.html

- `geturl-curl`: use the [curl] crate to implement HTTP requests. In order for
  this to take effect, you must use `--no-default-features` because
  `geturl-reqwest` is a default feature and it takes precedence
- `geturl-reqwest`: use the [reqwest] crate to implement HTTP requests (enabled
  by default)
- `native-tls-vendored`: if using [reqwest], activate the `vendored` option in
  the [native-tls] crate, causing OpenSSL to be vendored

[curl]: https://docs.rs/curl/
[reqwest]: https://docs.rs/reqwest/
[native-tls]: https://github.com/sfackler/rust-native-tls

## Network prefetching

Cached indexed-tar and `.ttb` network bundles remember the files used by earlier
builds. When those files are missing from the cache, they are fetched concurrently,
with `.ttb` files also decompressed in the worker threads. New file lookups use the
normal on-demand path; prefetching does not run in cache-only mode.

`TECTONIC_PREFETCH_CONCURRENCY` sets the number of workers (default: 16, maximum:
64). Each batch selects files within a 64 MiB budget, counting both compressed and
uncompressed sizes for `.ttb` files. Files beyond that budget remain available
through on-demand reads, as do files that could not be prefetched.
