use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg, Pat, PatIdent, Type};

#[derive(Debug, Clone)]
pub(crate) struct MethodInputs {
    pub(crate) self_arg: MethodSelfArg,
    pub(crate) args: Vec<MethodArg>,
}

impl MethodInputs {
    pub(crate) fn parse(inputs: Punctuated<FnArg, Token![,]>) -> Result<Self> {
        let span = inputs.span().unstable();
        let mut inputs_iter = inputs.into_iter();

        let self_arg = inputs_iter
            .next()
            .ok_or(Error::Empty)
            .and_then(|input| MethodSelfArg::parse(input))
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
    SelfRef(ArgSelfRef),
    SelfValue(ArgSelf),
    SelfCaptured(ArgCaptured),
}

impl MethodSelfArg {
    fn parse(arg: FnArg) -> Result<Self> {
        match arg {
            FnArg::SelfRef(self_ref) => Ok(MethodSelfArg::SelfRef(self_ref)),
            FnArg::SelfValue(self_value) => Ok(MethodSelfArg::SelfValue(self_value)),
            FnArg::Captured(ArgCaptured {
                pat:
                    Pat::Ident(PatIdent {
                        by_ref,
                        mutability,
                        ident,
                        subpat,
                    }),
                colon_token,
                ty,
            }) if ident == "self" => Ok(MethodSelfArg::SelfCaptured(ArgCaptured {
                pat: Pat::Ident(PatIdent {
                    by_ref,
                    mutability,
                    ident,
                    subpat,
                }),
                colon_token,
                ty,
            })),
            _ => Err(Error::Empty),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MethodArg {
    Captured(ArgCaptured),
    Ignored(Type),
}

impl MethodArg {
    pub(crate) fn parse(arg: FnArg) -> Result<Self> {
        let span = arg.span().unstable();
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
