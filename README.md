tracerr
=======

[![crates.io](https://img.shields.io/crates/v/tracerr.svg "crates.io")](https://crates.io/crates/tracerr)
[![Rust 1.75+](https://img.shields.io/badge/rustc-1.75+-lightgray.svg "Rust 1.75+")](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance)  
[![CI](https://github.com/instrumentisto/tracerr-rs/workflows/CI/badge.svg?branch=main "CI")](https://github.com/instrumentisto/tracerr-rs/actions?query=workflow%3ACI+branch%3Amain)
[![Rust docs](https://docs.rs/tracerr/badge.svg "Rust docs")](https://docs.rs/tracerr)

[API Docs](https://docs.rs/tracerr) |
[Changelog](https://github.com/instrumentisto/tracerr-rs/blob/main/CHANGELOG.md)

Custom compile-time captured error tracing for [Rust].




## Usage

The common rule:
- Use macro to capture trace frame in the invocation place.

```rust
use tracerr::Traced;

let err = tracerr::new!("my error"); // captures frame

let res: Result<(), _> = Err(err)
    .map_err(tracerr::wrap!()); // captures frame

let err: Traced<&'static str> = res.unwrap_err();
# #[cfg(not(target_os = "windows"))]
assert_eq!(
    format!("{}\n{}", err, err.trace()),
    r"my error
error trace:
rust_out
  at src/../README.md:7
rust_out
  at src/../README.md:10",
);

let (val, trace) = err.split();
# #[cfg(not(target_os = "windows"))]
assert_eq!(
    format!("{}\n{}", val, trace),
    r"my error
error trace:
rust_out
  at src/../README.md:7
rust_out
  at src/../README.md:10",
);
```




## License

Copyright © 2019-2024 Instrumentisto Team, <https://github.com/instrumentisto>

This software is subject to the terms of the [Blue Oak Model License 1.0.0](https://github.com/instrumentisto/tracerr-rs/blob/main/LICENSE.md). If a copy of the [BlueOak-1.0.0](https://spdx.org/licenses/BlueOak-1.0.0.html) license was not distributed with this file, You can obtain one at <https://blueoakcouncil.org/license/1.0.0>.




[Rust]: https://rust-lang.org
