use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ExprStruct};

#[proc_macro_derive(Notation)]
pub fn derive_notation(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::noteworthy::Notation for #name #ty_generics #where_clause {}
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn annotation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let notation_expr = syn::parse_macro_input!(attr as ExprStruct);
    let input = syn::parse_macro_input!(item as DeriveInput);
    let target_ident = &input.ident;
    let notation_path = &notation_expr.path;

    let expanded = quote! {
        #input

        impl ::noteworthy::Annotated<#notation_path> for #target_ident {
            const DATA: #notation_path = #notation_expr;
        }
    };

    expanded.into()
}
