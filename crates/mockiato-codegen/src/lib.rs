//! Codegen for `mockiato`. Do not use this crate directly.

#![recursion_limit = "128"]
#![cfg_attr(
    rustc_is_nightly,
    feature(
        proc_macro_diagnostic,
        proc_macro_span,
        proc_macro_hygiene,
        bind_by_move_pattern_guards,
        decl_macro,
        box_syntax,
        box_patterns
    )
)]
#![warn(clippy::dbg_macro, clippy::unimplemented)]
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

extern crate proc_macro;

mod constant;
mod diagnostic;
mod generate;
mod mockable;
mod parse;
mod result;
mod syn_ext;

use self::mockable::Mockable;
#[cfg(rustc_is_nightly)]
use crate::diagnostic::{Diagnostic, DiagnosticLevel, DiagnosticMessage};
use crate::result::Error;
#[cfg(not(rustc_is_nightly))]
use itertools::Itertools;
use proc_macro::TokenStream as ProcMacroTokenStream;
#[cfg(rustc_is_nightly)]
use proc_macro::{
    Diagnostic as ProcMacroDiagnostic, Level as ProcMacroLevel, Span as ProcMacroSpan,
};
#[cfg(rustc_is_nightly)]
use proc_macro2::Span;
use syn::{parse_macro_input, AttributeArgs, Item};

#[doc(hidden)]
#[proc_macro_attribute]
pub fn mockable(args: ProcMacroTokenStream, input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let original_input = input.clone();

    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let mockable = Mockable::new();

    match mockable.expand(attr, item) {
        Ok(output) => ProcMacroTokenStream::from(output),
        Err(error) => {
            emit_diagnostics(error);
            original_input
        }
    }
}

#[cfg(not(rustc_is_nightly))]
fn emit_diagnostics(error: Error) {
    let errors = error
        .diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.message)
        .join("\n");
    panic!("{}", errors);
}

#[cfg(rustc_is_nightly)]
fn emit_diagnostics(error: Error) {
    error
        .diagnostics
        .into_iter()
        .map(to_proc_macro_diagnostic)
        .for_each(ProcMacroDiagnostic::emit);
}

#[cfg(rustc_is_nightly)]
fn to_proc_macro_diagnostic(source: Diagnostic) -> ProcMacroDiagnostic {
    let level = to_proc_macro_level(source.level);
    let span = to_proc_macro_span(source.span);
    let diagnostic = ProcMacroDiagnostic::spanned(span, level, source.message);
    let diagnostic = add_notes_to_proc_macro_diagnostic(diagnostic, source.notes);
    add_help_to_proc_macro_diagnostic(diagnostic, source.help)
}

#[cfg(rustc_is_nightly)]
fn add_help_to_proc_macro_diagnostic(
    diagnostic: ProcMacroDiagnostic,
    help: Vec<DiagnosticMessage>,
) -> ProcMacroDiagnostic {
    help.into_iter()
        .fold(diagnostic, |diagnostic, help| match help.span {
            Some(span) => diagnostic.span_help(to_proc_macro_span(span), help.message),
            None => diagnostic.help(help.message),
        })
}

#[cfg(rustc_is_nightly)]
fn add_notes_to_proc_macro_diagnostic(
    diagnostic: ProcMacroDiagnostic,
    notes: Vec<DiagnosticMessage>,
) -> ProcMacroDiagnostic {
    notes
        .into_iter()
        .fold(diagnostic, |diagnostic, note| match note.span {
            Some(span) => diagnostic.span_note(to_proc_macro_span(span), note.message),
            None => diagnostic.note(note.message),
        })
}

#[cfg(rustc_is_nightly)]
fn to_proc_macro_span(span: Span) -> ProcMacroSpan {
    span.unstable()
}

#[cfg(rustc_is_nightly)]
fn to_proc_macro_level(level: DiagnosticLevel) -> ProcMacroLevel {
    match level {
        DiagnosticLevel::Error => ProcMacroLevel::Error,
    }
}
