use crate::constant::ATTR_NAME;
use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{AttributeArgs, Ident, Item};

#[derive(Default)]
pub(crate) struct Mockable;

impl Mockable {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn expand(&self, attr: AttributeArgs, item: Item) -> TokenStream {
        let mockable_attr = match MockableAttr::parse(attr).map_err(|err| err.emit(|d| d)) {
            Ok(mockable_attr) => mockable_attr,
            Err(_) => return TokenStream::new(),
        };

        let item_span = item.span().unstable();
        let trait_decl = match TraitDecl::parse(item.clone()).map_err(|err| {
            err.emit(|d| {
                d.span_note(
                    item_span,
                    format!(
                        "Required because of #[{}] on the trait declaration",
                        ATTR_NAME
                    ),
                )
            })
        }) {
            Ok(trait_decl) => trait_decl,
            Err(_) => return TokenStream::new(),
        };

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);

        TokenStream::from(quote! {
            #item

            #[derive(Debug)]
            struct #mock_struct_ident;
        })
    }
}

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.ident)
        .unwrap_or_else(|| Ident::new(&format!("{}Mock", trait_decl.ident), trait_decl.span.into()))
}
