use super::constant::arguments_ident;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::Ident;

pub(crate) fn generate_trait_impl(
    trait_decl: &TraitDecl,
    mock_struct_ident: &Ident,
    mod_ident: &Ident,
) -> TokenStream {
    let trait_ident = &trait_decl.ident;
    let unsafety = &trait_decl.unsafety;

    let method_impls: TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_method_impl(method_decl, mod_ident))
        .collect();

    quote! {
        #unsafety impl #trait_ident for #mock_struct_ident {
            #method_impls
        }
    }
}

fn generate_method_impl(method_decl: &MethodDecl, mod_ident: &Ident) -> TokenStream {
    let MethodDecl {
        ident,
        unsafety,
        generics,
        inputs,
        output,
        ..
    } = method_decl;

    let self_arg = &inputs.self_arg;
    let arguments: Punctuated<_, Token![,]> = method_decl.inputs.args.iter().collect();

    let where_clause = &generics.where_clause;

    let arguments_struct_ident = arguments_ident(ident);
    let arguments_struct_fields: Punctuated<_, Token![,]> = method_decl
        .inputs
        .args
        .iter()
        .map(|argument| &argument.ident)
        .collect();

    quote! {
        #unsafety fn #ident#generics(#self_arg, #arguments) #output #where_clause {
            self.#ident.call_unwrap(self::#mod_ident::#arguments_struct_ident { #arguments_struct_fields })
        }
    }
}
