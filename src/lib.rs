#![doc = include_str!("../README.md")]
#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::all,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::absolute_paths,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::as_ptr_cast_mut,
    clippy::assertions_on_result_states,
    clippy::branches_sharing_code,
    clippy::cfg_not_test,
    clippy::clear_with_drain,
    clippy::clone_on_ref_ptr,
    clippy::collection_is_never_read,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::empty_enum_variants_with_brackets,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::infinite_loop,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_over_hash_type,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::large_stack_frames,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::map_with_unused_argument_over_ranges,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::needless_pass_by_ref_mut,
    clippy::needless_raw_strings,
    clippy::non_zero_suggestions,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pathbuf_init_then_push,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::read_zero_byte_vec,
    clippy::redundant_clone,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::semicolon_inside_block,
    clippy::set_contains_or_insert,
    clippy::shadow_unrelated,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::too_long_first_doc_paragraph,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::uninhabited_references,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_struct_initialization,
    clippy::unneeded_field_pattern,
    clippy::unused_peekable,
    clippy::unused_result_ok,
    clippy::unused_trait_names,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::while_float,
    clippy::wildcard_enum_match_arm,
    explicit_outlives_requirements,
    future_incompatible,
    let_underscore_drop,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    redundant_lifetimes,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    unit_bindings,
    unnameable_types,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

mod trace;

use std::{
    error::Error,
    sync::atomic::{AtomicUsize, Ordering},
};

use derive_more::with_trait::{AsMut, AsRef, Display};
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
#[display("{err}")]
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
    #[must_use]
    pub fn into_inner(self) -> E {
        self.err
    }

    /// Splits this [`Traced`] wrapper into the underlying error and the
    /// captured [`Trace`].
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

// TODO: Use `#[warn(clippy::missing_trait_methods)]` once its more clever about
//       unstable methods.
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
/// into a [`Traced`] wrapper containing this [`Frame`].
///
/// If the error represents a [`Traced`] already, then just growths its
/// [`Trace`] with the captured [`Frame`].
///
/// # Example
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
/// required [`From`] conversion for the wrapped error.
///
/// If the error represents a [`Traced`] already, then just applies [`From`]
/// conversion and growths its [`Trace`] with the captured [`Frame`].
///
/// # Example
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
/// [`Frame`].
///
/// If the error represents a [`Traced`] already, then just growths its
/// [`Trace`] with the captured [`Frame`].
///
/// # Example
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
/// into a [`Traced`] wrapper with this [`Frame`].
///
/// If the error represents a [`Traced`] already, then just growths its
/// [`Trace`] with the captured [`Frame`].
///
/// # Example
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
