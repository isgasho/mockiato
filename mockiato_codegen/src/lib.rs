#![feature(
    quote,
    extern_crate_item_prelude,
    proc_macro_diagnostic,
    proc_macro_span,
    proc_macro_hygiene
)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

mod constant;
mod mockable;
mod parse;

use self::mockable::Mockable;
use proc_macro::TokenStream;
use syn::{AttributeArgs, Item};

pub(crate) type Result<T> = std::result::Result<T, ()>;

#[proc_macro_attribute]
pub fn mockable(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let mockable = Mockable::new();

    mockable.expand(attr, item)
}
