`tracerr` changelog
===================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.1.2] · 2020-11-03
[0.1.2]: /../../tree/v0.1.2

[Diff](/../../compare/v0.1.1...v0.1.2)

### Added

- `Clone` implementation for `Traced` ([#3](/../../pull/3)).




## [0.1.1] · 2019-11-22
[0.1.1]: /../../tree/v0.1.1

[Diff](/../../compare/v0.1.0...v0.1.1)

### Fixed

- [ICE](https://github.com/rust-lang/rust/issues/64450) when building on `wasm32-unknown-unknown` target ([#1](/../../pull/1)).




## [0.1.0] · 2019-11-20
[0.1.0]: /../../tree/v0.1.0

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





[Semantic Versioning 2.0.0]: https://semver.org
