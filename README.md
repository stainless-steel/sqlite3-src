# sqlite3-src [![Package][package-img]][package-url] [![Build][build-img]][build-url]

The package provides [SQLite].

The following Cargo features are supported:

* `bundled` to compile SQLite from the source code that comes with the package,
  ignoring any SQLite libraries that might be installed in the system.

It is also possible to enable various [compile-time options] by setting
environment variables with the same names, such as
`SQLITE_ENABLE_FTS5`.

## WebAssembly

WebAssembly is supported through [WASI SDK]. To compile for the `wasm32-wasip1`
or `wasm32-wasip2` target, define either

* the `sqlite3_src_wasi_sdk_path` configuration flag for `rustc` or
* the `WASI_SDK_PATH` environment variable

to point to an existing installation of the SDK. The path should be absolute.

To compile for the `wasm32-unknown-unknown` target, define in addition either

* the `sqlite3_src_wasi_version` configuration flag for `rustc` or
* the `WASI_VERSION` environment variable

to specify which WASI version to compile for.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[SQLite]: https://sqlite.org
[compile-time options]: https://www.sqlite.org/compile.html
[WASI SDK]: https://github.com/WebAssembly/wasi-sdk/releases

[build-img]: https://github.com/stainless-steel/sqlite3-src/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/sqlite3-src/actions/workflows/build.yml
[package-img]: https://img.shields.io/crates/v/sqlite3-src.svg
[package-url]: https://crates.io/crates/sqlite3-src
