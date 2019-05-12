//! Minimalistic mocking framework, ready for Rust 2018
//!
//! # Examples
//!
//! ```
//! use mockiato::{mockable, partial_eq};
//!
//! #[cfg_attr(test, mockable)]
//! trait Greeter {
//!     fn greet(&self, name: &str) -> String;
//! }
//!
//! let mut greeter = GreeterMock::new();
//!
//! greeter.expect_greet(partial_eq("Jane"))
//!        .times(1..)
//!        .returns(String::from("Hello Jane"));
//!
//! assert_eq!("Hello Jane", greeter.greet("Jane"));
//! ```

#![feature(specialization)]
#![feature(doc_cfg)]
#![warn(missing_docs, clippy::dbg_macro, clippy::unimplemented)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

#[cfg(not(rustdoc))]
pub use mockiato_codegen::mockable;

#[cfg(rustdoc)]
#[macro_export]
/// Generates a mock struct from a trait.
///
/// # Parameters
///
/// ## `static_references`
/// Forces values stored in argument matchers to be `'static`. This is used when the mock needs to satisfy
/// `'static` e.g. when downcasting the mocked trait to a concrete implementation using the `Any` trait.
///
/// ```
/// use std::any::Any;
/// use mockiato::mockable;
///
/// #[cfg_attr(test, mockable(static_references))]
/// pub trait Animal: Any {
///     fn make_sound(&self);
/// }
/// ```
///
/// ## `name`
/// Allows customizing the mock struct's name.
/// ```
/// use mockiato::mockable;
///
/// #[cfg_attr(test, mockable(name = "CuteAnimalMock"))]
/// trait Animal {
///    fn make_sound(&self);
/// }
/// ```
macro_rules! mockable {
    () => {};
}

pub use crate::internal::expected_calls::ExpectedCalls;
pub use crate::internal::matcher::any::any;
pub use crate::internal::matcher::nearly_eq::{nearly_eq, nearly_eq_with_accuracy};
pub use crate::internal::matcher::partial_eq::{partial_eq, partial_eq_owned};
pub use crate::internal::matcher::unordered_vec_eq::unordered_vec_eq;
pub use crate::internal::MethodCallBuilder;

#[doc(hidden)]
pub mod internal;
