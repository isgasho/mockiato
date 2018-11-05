use crate::spanned::SpannedUnstable;
use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level};
use syn::punctuated::Punctuated;
use syn::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg, Pat, PatIdent, Type};

#[derive(Debug, Clone)]
pub(crate) struct MethodInputs {
    pub(crate) self_arg: MethodSelfArg,
    pub(crate) args: Vec<MethodArg>,
}

impl MethodInputs {
    pub(crate) fn parse(inputs: Punctuated<FnArg, Token![,]>) -> Result<Self> {
        let span = inputs.span_unstable();
        let mut inputs_iter = inputs.into_iter();

        let self_arg = inputs_iter
            .next()
            .ok_or(())
            .and_then(MethodSelfArg::parse)
            .map_err(|_| {
                Error::Diagnostic(Diagnostic::spanned(
                    span,
                    Level::Error,
                    "The first parameter of a method must be self, so that the trait is object-safe",
                ))
            })?;

        let args = inputs_iter.map(MethodArg::parse);

        Ok(Self {
            self_arg,
            args: merge_results!(args),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MethodSelfArg {
    /// `self` is taken by reference: `&self` or `&mut self`
    Ref(ArgSelfRef),
    /// `self` is consumed: `self`
    Value(ArgSelf),
    /// `self` has a type. Example: `self: Box<Self>`
    Captured(Box<ArgCaptured>),
}

impl MethodSelfArg {
    fn parse(arg: FnArg) -> std::result::Result<Self, ()> {
        match arg {
            FnArg::SelfRef(self_ref) => Ok(MethodSelfArg::Ref(self_ref)),
            FnArg::SelfValue(self_value) => Ok(MethodSelfArg::Value(self_value)),
            FnArg::Captured(captured_arg) => Self::parse_captured_arg(captured_arg),
            _ => Err(()),
        }
    }

    fn parse_captured_arg(arg: ArgCaptured) -> std::result::Result<Self, ()> {
        const SELF_ARG_NAME: &str = "self";

        match arg.pat {
            Pat::Ident(PatIdent { ref ident, .. }) if ident == SELF_ARG_NAME => {
                Ok(MethodSelfArg::Captured(box arg))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MethodArg {
    /// A "normal" argument. Example: `first_name: &str`
    Captured(ArgCaptured),
    /// An ignored argument. Example: `_: String`
    Ignored(Type),
}

impl MethodArg {
    pub(crate) fn parse(arg: FnArg) -> Result<Self> {
        let span = arg.span_unstable();
        match arg {
            FnArg::Captured(captured) => Ok(MethodArg::Captured(captured)),
            FnArg::Ignored(ty) => Ok(MethodArg::Ignored(ty)),
            _ => Err(Error::Diagnostic(Diagnostic::spanned(
                span,
                Level::Error,
                "Only captured and ignored method arguments are supported",
            ).note("This error should never appear, because rustc already enforces these requirements"))),
        }
    }
}
