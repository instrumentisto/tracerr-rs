`tracerr` changelog
===================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## main

[Diff](https://github.com/instrumentisto/tracerr-rs/compare/v0.4.0...main)

### BC Breaks

- Set [MSRV] to [1.85.0](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html). ([bd17ccb2])

[bd17ccb2]: https://github.com/instrumentisto/tracerr-rs/commit/bd17ccb23ee90ff81900d51ca86d7056f96a7f81




## [0.4.0] · 2025-02-04
[0.4.0]: https://github.com/instrumentisto/tracerr-rs/tree/v0.4.0

[Diff](https://github.com/instrumentisto/tracerr-rs/compare/v0.3.0...v0.4.0)

### BC Breaks

- Set [MSRV] to [1.75.0](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html). ([#11])

### Updated

- [`derive_more` crate] to 2.0 version. ([#11], [#12], [#14])
- [`sealed` crate] to 0.6 version. ([#13])

[#11]: https://github.com/instrumentisto/tracerr-rs/pull/11
[#12]: https://github.com/instrumentisto/tracerr-rs/pull/12
[#13]: https://github.com/instrumentisto/tracerr-rs/pull/13
[#14]: https://github.com/instrumentisto/tracerr-rs/pull/14




## [0.3.0] · 2021-10-27
[0.3.0]: https://github.com/instrumentisto/tracerr-rs/tree/v0.3.0

[Diff](https://github.com/instrumentisto/tracerr-rs/compare/v0.2.0...v0.3.0)

### BC Breaks

- Renamed `Traced::from_parts()` to `Traced::compose()`.
- Renamed `Traced::into_parts()` to `Traced::split()`.
- Set [MSRV] to [1.56.0](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html).

### Added

- Sealing `WrapTraced` trait with [`#[sealed]`](https://docs.rs/sealed).




## [0.2.0] · 2021-06-24
[0.2.0]: https://github.com/instrumentisto/tracerr-rs/tree/v0.2.0

[Diff](https://github.com/instrumentisto/tracerr-rs/compare/v0.1.2...v0.2.0)

### BC Breaks

- Change `Traced::from_parts()` arguments to `(err: E, trace: Trace)` ([#4]).
- Remove `failure` support ([9f87f0b9], [#2]).

### Added

- `From<(E, Trace)>` implementation for `Traced<E>` ([#4]).

[#2]: https://github.com/instrumentisto/tracerr-rs/pull/2
[#4]: https://github.com/instrumentisto/tracerr-rs/pull/4
[9f87f0b9]: https://github.com/instrumentisto/tracerr-rs/commit/9f87f0b9ff6565d02c28fe1a2a8a34927bb447c6




## [0.1.2] · 2020-11-03
[0.1.2]: https://github.com/instrumentisto/tracerr-rs/tree/v0.1.2

[Diff](https://github.com/instrumentisto/tracerr-rs/compare/v0.1.1...v0.1.2)

### Added

- `Clone` implementation for `Traced` ([#3](https://github.com/instrumentisto/tracerr-rs/pull/3)).




## [0.1.1] · 2019-11-22
[0.1.1]: https://github.com/instrumentisto/tracerr-rs/tree/v0.1.1

[Diff](https://github.com/instrumentisto/tracerr-rs/compare/v0.1.0...v0.1.1)

### Fixed

- [ICE](https://github.com/rust-lang/rust/issues/64450) when building on `wasm32-unknown-unknown` target ([#1]).

[#1]: https://github.com/instrumentisto/tracerr-rs/pull/1




## [0.1.0] · 2019-11-20
[0.1.0]: https://github.com/instrumentisto/tracerr-rs/tree/v0.1.0

Published initial implementation, which provides:
- `Frame` and `Trace` types to represent error's trace;
- `Traced` wrapper for errors, which is able to carry and grow `Trace`;
- Macros for `Frame` capturing to use in user code:
    - `new!()` wraps error;
    - `map_from_and_new!()` wraps error and does `From` conversion for it;
    - `wrap!()` wraps error in a closure;
    - `map_from_and_wrap!()` wraps error and does `From` conversion for it in a closure;
    - `from_and_wrap!()` does `From` conversion for error and then wraps it in a closure.
- `map_from()` function, to apply `From` conversion for the error inside `Traced` without capturing the `Frame`.




[`derive_more` crate]: https://docs.rs/derive_more
[`sealed` crate]: https://docs.rs/sealed
[MSRV]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
[Semantic Versioning 2.0.0]: https://semver.org
