mod codegen;
mod parse;

use proc_macro::TokenStream;

#[proc_macro_derive(Outcome, attributes(outcome, from))]
pub fn derive_outcome(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match parse::parse(&input) {
        Ok(model) => codegen::codegen(&model).into(),
        Err(err) => err.to_compile_error().into(),
    }
}
