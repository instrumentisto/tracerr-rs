`tracerr` changelog
===================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.1.0] · 2019-11-20
[0.1.0]: /../../tree/0.1.0

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
