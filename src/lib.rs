#![doc = include_str!("../README.md")]
#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::as_conversions,
    clippy::branches_sharing_code,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::else_if_without_else,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::iter_with_drain,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::only_used_in_recursion,
    clippy::option_if_let_else,
    clippy::panic_in_result_fn,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    future_incompatible,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

mod trace;

use std::{
    error::Error,
    sync::atomic::{AtomicUsize, Ordering},
};

use derive_more::{AsMut, AsRef, Display};
use sealed::sealed;

#[doc(inline)]
pub use self::trace::*;

/// Default capacity for a [`Trace`] buffer initialization.
///
/// May be changed if your application requires larger size for better
/// performance and re-allocation avoidance.
pub static DEFAULT_FRAMES_CAPACITY: AtomicUsize = AtomicUsize::new(10);

/// Wrapper for an arbitrary error holding the captured error trace along.
#[derive(AsMut, AsRef, Clone, Debug, Display)]
// TODO: Use "{err}" syntax once MSRV bumps above 1.58, and `derive_more`
//       supports it.
#[display(fmt = "{}", err)]
pub struct Traced<E: ?Sized> {
    /// Captured error trace.
    trace: Trace,

    /// Original error.
    #[as_mut]
    #[as_ref]
    err: E,
}

impl<E: ?Sized> Traced<E> {
    /// References to the captured [`Trace`].
    ///
    /// This is a raw equivalent of `AsRef<Trace>` (which cannot be implemented
    /// at the moment due to the lack of specialization in Rust).
    #[must_use]
    pub const fn trace(&self) -> &Trace {
        &self.trace
    }
}

impl<E> Traced<E> {
    /// Destructs this [`Traced`] wrapper returning only the underlying error
    /// and loosing the captured [`Trace`].
    // false positive: constant functions cannot evaluate destructors
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_inner(self) -> E {
        self.err
    }

    /// Splits this [`Traced`] wrapper into the underlying error and the
    /// captured [`Trace`].
    // false positive: constant functions cannot evaluate destructors
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn split(self) -> (E, Trace) {
        (self.err, self.trace)
    }

    /// Composes the given error and the captured [`Trace`] into a [`Traced`]
    /// wrapper.
    #[must_use]
    pub const fn compose(error: E, trace: Trace) -> Self {
        Self { err: error, trace }
    }
}

impl<E> From<(E, Frame)> for Traced<E> {
    fn from((err, f): (E, Frame)) -> Self {
        err.wrap_traced(f)
    }
}

impl<E> From<(E, Trace)> for Traced<E> {
    fn from((err, trace): (E, Trace)) -> Self {
        Self::compose(err, trace)
    }
}

// TODO: Use when Rust will allow specialization... T_T
/*
impl<E> AsRef<Trace> for Traced<E> {
    fn as_ref(&self) -> &Trace {
        &self.trace
    }
}
*/

impl<E: Error + ?Sized> Error for Traced<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.err.source()
    }
}

/// Trait for wrapping errors into a [`Traced`] wrapper and growing its
/// [`Trace`] inside.
///
/// # Sealed
///
/// This trait is exposed only for being available inside macro invocations,
/// so, outside this library in any code the following MUST BE met:
/// - NEITHER this trait is implemented directly;
/// - NOR its methods are invoked directly.
#[sealed]
pub trait WrapTraced<E> {
    /// Wraps this error into a [`Traced`] wrapper, storing the given [`Frame`]
    /// of a [`Trace`] inside.
    #[must_use]
    fn wrap_traced(self, f: Frame) -> Traced<E>;
}

#[sealed]
impl<E> WrapTraced<E> for E {
    fn wrap_traced(self, f: Frame) -> Traced<Self> {
        let mut trace = Trace::new(Vec::with_capacity(
            DEFAULT_FRAMES_CAPACITY.load(Ordering::Relaxed),
        ));
        trace.push(f);
        Traced { err: self, trace }
    }
}

#[sealed]
impl<E> WrapTraced<E> for Traced<E> {
    /// Pushes the given [`Frame`] into the already captured [`Trace`] of this
    /// [`Traced`] wrapper.
    fn wrap_traced(mut self, f: Frame) -> Self {
        self.trace.push(f);
        self
    }
}

// TODO: deprecate when Rust will allow specialization
/// Maps an inner value of an error wrapped in a [`Traced`] with its [`From`]
/// implementation.
///
/// This is an equivalent of
/// `impl<E1, E2: From<E1>> From<Traced<E1>> for Traced<E2>`
/// (which cannot be implemented at the moment due to the lack of specialization
/// in Rust).
#[must_use]
pub fn map_from<F, T: From<F>>(e: Traced<F>) -> Traced<T> {
    Traced {
        err: T::from(e.err),
        trace: e.trace,
    }
}

// TODO: Use when Rust will allow specialization... T_T
/*
impl<E1, E2> From<Traced<E1>> for Traced<E2>
where
    E2: From<E1>,
{
    fn from(e: Traced<E1>) -> Self {
        unimplemented!()
    }
}
*/

/// Captures a new [`Frame`] in the invocation place and wraps the given error
/// into a [`Traced`] wrapper containing this [`Frame`]. If the error represents
/// a [`Traced`] already, then just growths its [`Trace`] with the captured
/// [`Frame`].
///
/// # Examples
///
/// ```rust
/// use tracerr::Traced;
///
/// let err: u32 = 89;
/// let err: Traced<u32> = tracerr::new!(err);
/// let err: Traced<u32> = tracerr::new!(err);
/// ```
#[macro_export]
macro_rules! new {
    ($e:expr) => {
        $crate::WrapTraced::wrap_traced($e, $crate::new_frame!())
    };
}

/// Captures a new [`Frame`] in the invocation place and wraps the given error
/// into a [`Traced`] wrapper containing this [`Frame`] with applying the
/// required [`From`] conversion for the wrapped error. If the error represents
/// a [`Traced`] already, then just applies [`From`] conversion and growths its
/// [`Trace`] with the captured [`Frame`].
///
/// # Examples
///
/// ```rust
/// use tracerr::Traced;
///
/// let err: Traced<u8> = tracerr::new!(8);
/// let err: Traced<u64> = tracerr::map_from_and_new!(err);
/// ```
#[macro_export]
macro_rules! map_from_and_new {
    ($e:expr) => {
        $crate::new!($crate::map_from($e))
    };
}

/// Provides a closure, which captures a new [`Frame`] in the invocation place
/// and wraps the given error into a [`Traced`] wrapper containing this
/// [`Frame`]. If the error represents a [`Traced`] already, then just growths
/// its [`Trace`] with the captured [`Frame`].
///
/// # Examples
///
/// ```rust
/// use tracerr::Traced;
///
/// let res: Result<(), u32> = Err(89);
/// let err: Traced<u32> = res
///     .map_err(tracerr::wrap!())
///     .map_err(tracerr::wrap!())
///     .unwrap_err();
/// ```
#[macro_export]
macro_rules! wrap {
    () => ($crate::wrap!(_ => _));
    ($from:ty) => ($crate::wrap!($from => _));
    (=> $to:ty) => ($crate::wrap!(_ => $to));
    ($from:ty => $to:ty) => {
        |err: $from| -> $crate::Traced<$to> {
            $crate::new!(err)
        }
    };
}

/// Provides a closure, which captures a new [`Frame`] in the invocation place
/// for the given [`Traced`] wrapper and applies the required [`From`]
/// conversion for the wrapped error.
///
/// # Examples
///
/// ```rust
/// use tracerr::Traced;
///
/// let res: Result<(), Traced<u8>> = Err(tracerr::new!(7));
/// let err: Traced<u64> =
///     res.map_err(tracerr::map_from_and_wrap!()).unwrap_err();
/// ```
#[macro_export]
macro_rules! map_from_and_wrap {
    () => ($crate::map_from_and_wrap!(_ => _));
    ($from:ty) => ($crate::map_from_and_wrap!($from => _));
    (=> $to:ty) => ($crate::map_from_and_wrap!(_ => $to));
    ($from:ty => $to:ty) => {
        |err: $crate::Traced<$from>| -> $crate::Traced<$to> {
            $crate::new!($crate::map_from::<$from, $to>(err))
        }
    };
}

/// Provides a closure, which captures a new [`Frame`] in the invocation place,
/// applies the required [`From`] conversion for the given error, and wraps it
/// into a [`Traced`] wrapper containing this [`Frame`]. If the error represents
/// a [`Traced`] already, then just growths its [`Trace`] with the captured
/// [`Frame`].
///
/// # Examples
///
/// ```rust
/// use tracerr::Traced;
///
/// let res: Result<(), u8> = Err(7);
/// let err: Traced<u64> = res.map_err(tracerr::from_and_wrap!()).unwrap_err();
/// ```
#[macro_export]
macro_rules! from_and_wrap {
    () => ($crate::from_and_wrap!(_ => _));
    ($from:ty) => ($crate::from_and_wrap!($from => _));
    (=> $to:ty) => ($crate::from_and_wrap!(_ => $to));
    ($from:ty => $to:ty) => {
        |err: $from| -> $crate::Traced<$to> {
            $crate::map_from::<$from, $to>($crate::new!(err))
        }
    };
}

#[cfg(test)]
mod new_macro_spec {
    use super::Traced;

    #[test]
    fn creates_new_trace_frame_on_initialization() {
        let err = super::new!(());
        assert_eq!(err.trace.len(), 1, "creates initial frame");
    }

    #[test]
    fn fills_trace_on_subsequent_calls() {
        let err = super::new!(());
        let err = super::new!(err);
        let err = super::new!(err);
        let err: Traced<()> = super::new!(err);
        assert_eq!(err.trace.len(), 4, "trace growths with each call");
    }
}

#[cfg(test)]
mod map_from_and_new_macro_spec {
    use super::Traced;

    #[test]
    fn fills_trace_on_subsequent_calls() {
        let err = super::new!(());
        let err = super::map_from_and_new!(err);
        let err = super::map_from_and_new!(err);
        let err: Traced<()> = super::map_from_and_new!(err);
        assert_eq!(err.trace.len(), 4, "trace growths with each call");
    }

    #[test]
    fn applies_required_from_implementation() {
        let err = super::new!(4u8);
        let err: Traced<u32> = super::map_from_and_new!(err);
        assert!(!err.trace.is_empty(), "captures frames");
    }
}

#[cfg(test)]
mod wrap_macro_spec {
    use super::Traced;

    #[test]
    fn creates_new_trace_frame_on_initialization() {
        let res: Result<(), ()> = Err(());
        let err = res.map_err(super::wrap!()).unwrap_err();
        assert_eq!(err.trace.len(), 1, "creates initial frame");
    }

    #[test]
    fn fills_trace_on_subsequent_calls() {
        let res: Result<(), ()> = Err(());
        let res = res.map_err(super::wrap!());
        let res = res.map_err(super::wrap!());
        let res = res.map_err(super::wrap!(Traced<_>));
        let err = res.map_err(super::wrap!(=> ())).unwrap_err();
        assert_eq!(err.trace.len(), 4, "trace growths with each call");
    }
}

#[cfg(test)]
mod map_from_and_wrap_macro_spec {
    use super::Traced;

    #[test]
    fn fills_trace_on_subsequent_calls() {
        let res: Result<(), ()> = Err(());
        let res = res.map_err(super::wrap!());
        let res = res.map_err(super::map_from_and_wrap!());
        let res = res.map_err(super::map_from_and_wrap!());
        let err = res.map_err(super::map_from_and_wrap!(=> ())).unwrap_err();
        assert_eq!(err.trace.len(), 4, "trace growths with each call");
    }

    #[test]
    fn applies_required_from_implementation() {
        let res: Result<(), u8> = Err(54);
        let res = res.map_err(super::wrap!());
        let err: Traced<u64> =
            res.map_err(super::map_from_and_wrap!()).unwrap_err();
        assert!(!err.trace.is_empty(), "captures frames");
    }
}

#[cfg(test)]
mod from_and_wrap_macro_spec {
    use super::Traced;

    #[test]
    fn fills_trace_on_subsequent_calls() {
        let res: Result<(), ()> = Err(());
        let res = res.map_err(super::wrap!());
        let res = res.map_err(super::from_and_wrap!());
        let res = res.map_err(super::from_and_wrap!());
        let err = res.map_err(super::from_and_wrap!(=> ())).unwrap_err();
        assert_eq!(err.trace.len(), 4, "trace growths with each call");
    }

    #[test]
    fn applies_required_from_implementation() {
        let res: Result<(), u8> = Err(54);
        let err: Traced<u64> =
            res.map_err(super::from_and_wrap!()).unwrap_err();
        assert!(!err.trace.is_empty(), "captures frames");
    }
}
