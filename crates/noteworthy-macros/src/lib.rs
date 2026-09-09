use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{DeriveInput, Expr, Path};

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
    let input = syn::parse_macro_input!(item as DeriveInput);
    let target_ident = &input.ident;

    let attr_ts: TokenStream2 = attr.into();

    // Split into path tokens and optional group.
    let (path_ts, group) = split_path_and_group(&attr_ts);
    let path: Path = syn::parse2(path_ts).expect("annotation must start with a path");

    let data_expr = match group {
        Some(TokenTree::Group(g)) => {
            let inner = g.stream();
            let fixed = rewrite_bare_rest(&inner, &quote! { #path::default() });
            Expr::Verbatim(quote! { (#path { #fixed }) })
        }
        None => Expr::Verbatim(quote! { (#path { ..#path::default() }) }),
        _ => unreachable!(),
    };

    let expanded = quote! {
        #input

        impl ::noteworthy::Annotated<#path> for #target_ident {
            fn data() -> #path { #data_expr }
        }
    };

    expanded.into()
}

/// Split a token stream into path tokens and the first group.
fn split_path_and_group(ts: &TokenStream2) -> (TokenStream2, Option<TokenTree>) {
    let mut path_tokens = TokenStream2::new();
    let mut group = None;
    for token in ts.clone().into_iter() {
        if group.is_some() {
            continue;
        }
        match &token {
            TokenTree::Group(_) => group = Some(token),
            _ => path_tokens.extend(std::iter::once(token)),
        }
    }
    (path_tokens, group)
}

/// Rewrite bare `..` (followed by `}`, `,`, or end of stream) to
/// `..Path::default()`. Leaves `..expr` (with a base) untouched.
fn rewrite_bare_rest(ts: &TokenStream2, default_expr: &TokenStream2) -> TokenStream2 {
    let tokens: Vec<TokenTree> = ts.clone().into_iter().collect();
    let mut result = TokenStream2::new();
    let mut i = 0;

    while i < tokens.len() {
        if is_dot(&tokens[i])
            && i + 1 < tokens.len()
            && is_dot(&tokens[i + 1])
            && is_bare_rest_after(&tokens, i + 2)
        {
            result.extend(quote! { ..#default_expr });
            i += 2;
        } else {
            result.extend(std::iter::once(tokens[i].clone()));
            i += 1;
        }
    }

    result
}

fn is_dot(token: &TokenTree) -> bool {
    matches!(token, TokenTree::Punct(p) if p.as_char() == '.')
}

fn is_bare_rest_after(tokens: &[TokenTree], i: usize) -> bool {
    match tokens.get(i) {
        None => true,
        Some(TokenTree::Punct(p)) if p.as_char() == '}' => true,
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => true,
        Some(TokenTree::Group(_)) => true,
        _ => false,
    }
}
