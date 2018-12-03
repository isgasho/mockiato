#![feature(specialization)]
#![deny(clippy::unimplemented)]

pub use mockiato_codegen::mockable;

pub use crate::internal::{partial_eq, partial_eq_owned};
pub use crate::internal::ExpectedCalls;
pub use crate::internal::MethodCallBuilder;

#[doc(hidden)]
pub mod internal;
