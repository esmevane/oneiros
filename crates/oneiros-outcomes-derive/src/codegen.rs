use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::LitStr;

use crate::parse::{FieldModel, FormatArgs, Level, OutcomeConfig, OutcomeModel, VariantModel};

/// Generate all trait implementations from the parsed model.
pub fn codegen(model: &OutcomeModel) -> TokenStream {
    let reportable_impl = gen_reportable(model);
    let from_impls = gen_from_impls(model);

    quote! {
        #reportable_impl
        #from_impls
    }
}

/// Generate the `Reportable` impl for the enum.
fn gen_reportable(model: &OutcomeModel) -> TokenStream {
    let ident = &model.ident;
    let (impl_generics, ty_generics, where_clause) = model.generics.split_for_impl();

    let level_arms = model.variants.iter().map(gen_level_arm);
    let message_arms = model.variants.iter().map(gen_message_arm);
    let log_message_arms = model.variants.iter().map(gen_log_message_arm);
    let prompt_arms = model.variants.iter().map(gen_prompt_arm);

    // Only override log_message if any variant has an explicit log.
    let has_any_log = model.variants.iter().any(|v| match &v.config {
        OutcomeConfig::Explicit { log, .. } => log.is_some(),
        OutcomeConfig::Transparent { .. } => true,
    });

    // Only override prompt if any variant has an explicit prompt.
    let has_any_prompt = model.variants.iter().any(|v| match &v.config {
        OutcomeConfig::Explicit { prompt, .. } => prompt.is_some(),
        OutcomeConfig::Transparent { .. } => true,
    });

    let log_message_method = if has_any_log {
        quote! {
            fn log_message(&self) -> String {
                match self {
                    #(#log_message_arms)*
                }
            }
        }
    } else {
        TokenStream::new()
    };

    let prompt_method = if has_any_prompt {
        quote! {
            fn prompt(&self) -> Option<String> {
                match self {
                    #(#prompt_arms)*
                }
            }
        }
    } else {
        TokenStream::new()
    };

    quote! {
        impl #impl_generics oneiros_outcomes::Reportable for #ident #ty_generics #where_clause {
            fn level(&self) -> tracing::Level {
                match self {
                    #(#level_arms)*
                }
            }

            fn message(&self) -> String {
                match self {
                    #(#message_arms)*
                }
            }

            #log_message_method
            #prompt_method
        }
    }
}

/// Generate the destructuring pattern for a variant's fields.
fn destructure_pattern(variant: &VariantModel) -> TokenStream {
    let ident = &variant.ident;
    if variant.fields.is_empty() {
        quote! { Self::#ident }
    } else {
        let bindings = variant.fields.iter().map(field_binding);
        // Check if fields are named or positional.
        match &variant.fields[0].member {
            syn::Member::Named(_) => quote! { Self::#ident { #(#bindings),* } },
            syn::Member::Unnamed(_) => quote! { Self::#ident(#(#bindings),*) },
        }
    }
}

/// Generate the binding name for a field.
fn field_binding(field: &FieldModel) -> TokenStream {
    match &field.member {
        syn::Member::Named(ident) => quote! { #ident },
        syn::Member::Unnamed(idx) => {
            let var = format_ident!("_{}", idx.index);
            quote! { #var }
        }
    }
}

/// Rewrite a format string, replacing `{0}` → `{_0}`, `{1}` → `{_1}`, etc.
/// Leaves named references like `{name}` and format specs like `{0:?}` intact
/// (the named ones are already valid bindings from destructuring, and the
/// positional ones just need the `_` prefix to match our binding names).
fn rewrite_format_str(fmt: &LitStr) -> LitStr {
    let value = fmt.value();
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            result.push('{');
            // Check for escaped brace `{{`
            if chars.peek() == Some(&'{') {
                result.push(chars.next().unwrap());
                continue;
            }
            // Check for a digit (positional field reference)
            if chars.peek().is_some_and(|c| c.is_ascii_digit()) {
                result.push('_');
                // Let the rest of the content through (digits, format specs, etc.)
            }
        } else {
            result.push(ch);
        }
    }

    LitStr::new(&result, fmt.span())
}

/// Generate a `format!(...)` call from a `FormatArgs`.
/// Rewrites positional references in the format string (`{0}` → `{_0}`).
/// Dot-prefix rewriting (`.0` → `_0`) is handled during parsing.
fn format_call(fmt_args: &FormatArgs) -> TokenStream {
    let fmt = rewrite_format_str(&fmt_args.fmt);
    let args = &fmt_args.args;

    if args.is_empty() {
        quote! { format!(#fmt) }
    } else {
        quote! { format!(#fmt, #(#args),*) }
    }
}

fn gen_level_arm(variant: &VariantModel) -> TokenStream {
    let pat = destructure_pattern(variant);

    match &variant.config {
        OutcomeConfig::Transparent { .. } => {
            let inner = field_binding(&variant.fields[0]);
            quote! { #pat => #inner.level(), }
        }
        OutcomeConfig::Explicit { level, .. } => {
            let level_token = level_to_tokens(*level);
            quote! { #pat => #level_token, }
        }
    }
}

fn gen_message_arm(variant: &VariantModel) -> TokenStream {
    let pat = destructure_pattern(variant);

    match &variant.config {
        OutcomeConfig::Transparent { .. } => {
            let inner = field_binding(&variant.fields[0]);
            quote! { #pat => #inner.message(), }
        }
        OutcomeConfig::Explicit { message, .. } => {
            let call = format_call(message);
            quote! { #pat => #call, }
        }
    }
}

fn gen_log_message_arm(variant: &VariantModel) -> TokenStream {
    let pat = destructure_pattern(variant);

    match &variant.config {
        OutcomeConfig::Transparent { .. } => {
            let inner = field_binding(&variant.fields[0]);
            quote! { #pat => #inner.log_message(), }
        }
        OutcomeConfig::Explicit { log, message, .. } => {
            let call = match log {
                Some(log_fmt) => format_call(log_fmt),
                None => format_call(message),
            };
            quote! { #pat => #call, }
        }
    }
}

fn gen_prompt_arm(variant: &VariantModel) -> TokenStream {
    let pat = destructure_pattern(variant);

    match &variant.config {
        OutcomeConfig::Transparent { .. } => {
            let inner = field_binding(&variant.fields[0]);
            quote! { #pat => #inner.prompt(), }
        }
        OutcomeConfig::Explicit { prompt, .. } => match prompt {
            Some(prompt_fmt) => {
                let call = format_call(prompt_fmt);
                quote! { #pat => Some(#call), }
            }
            None => {
                quote! { #pat => None, }
            }
        },
    }
}

fn level_to_tokens(level: Level) -> TokenStream {
    match level {
        Level::Trace => quote! { tracing::Level::TRACE },
        Level::Debug => quote! { tracing::Level::DEBUG },
        Level::Info => quote! { tracing::Level::INFO },
        Level::Warn => quote! { tracing::Level::WARN },
        Level::Error => quote! { tracing::Level::ERROR },
    }
}

/// Generate `From` impls for every `#[from]` field.
fn gen_from_impls(model: &OutcomeModel) -> TokenStream {
    let ident = &model.ident;

    let impls: Vec<_> = model
        .variants
        .iter()
        .flat_map(|v| v.fields.iter().filter(|f| f.is_from).map(move |f| (v, f)))
        .map(|(v, f)| {
            let (impl_generics, ty_generics, where_clause) = model.generics.split_for_impl();
            let variant_ident = &v.ident;
            let from_ty = &f.ty;

            let body = match &f.member {
                syn::Member::Named(name) => {
                    quote! { #ident::#variant_ident { #name: value } }
                }
                syn::Member::Unnamed(_) => {
                    quote! { #ident::#variant_ident(value) }
                }
            };

            quote! {
                impl #impl_generics From<#from_ty> for #ident #ty_generics #where_clause {
                    fn from(value: #from_ty) -> Self {
                        #body
                    }
                }
            }
        })
        .collect();

    quote! { #(#impls)* }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    fn pretty(tokens: TokenStream) -> String {
        let file = syn::parse2::<syn::File>(tokens).expect("generated code should be valid Rust");
        prettyplease::unparse(&file)
    }

    fn codegen_from_input(input: TokenStream) -> String {
        let derive_input: syn::DeriveInput = syn::parse2(input).unwrap();
        let model = parse::parse(&derive_input).unwrap();
        pretty(codegen(&model))
    }

    #[test]
    fn simple_unit_variant() {
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("hello world"))]
                Simple,
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Simple => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Simple => format!("hello world"),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn tuple_variant_with_positional_args() {
        // User writes {0}, {1} — codegen rewrites to {_0}, {_1}
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("found {0} at {1}"))]
                Found(String, String),
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Found(_0, _1) => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Found(_0, _1) => format!("found {_0} at {_1}"),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn variant_with_custom_level() {
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("bad thing"), level = "warn")]
                Bad,
                #[outcome(message("good thing"))]
                Good,
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Bad => tracing::Level::WARN,
                        Self::Good => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Bad => format!("bad thing"),
                        Self::Good => format!("good thing"),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn variant_with_log_and_prompt() {
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(
                    message("initialized {0}"),
                    log("tenant {0} init complete"),
                    prompt("run doctor next")
                )]
                Done(String),
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Done(_0) => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Done(_0) => format!("initialized {_0}"),
                    }
                }
                fn log_message(&self) -> String {
                    match self {
                        Self::Done(_0) => format!("tenant {_0} init complete"),
                    }
                }
                fn prompt(&self) -> Option<String> {
                    match self {
                        Self::Done(_0) => Some(format!("run doctor next")),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn transparent_variant() {
        let actual = codegen_from_input(quote! {
            enum Outer {
                #[outcome(transparent)]
                Inner(InnerOutcome),
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for Outer {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Inner(_0) => _0.level(),
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Inner(_0) => _0.message(),
                    }
                }
                fn log_message(&self) -> String {
                    match self {
                        Self::Inner(_0) => _0.log_message(),
                    }
                }
                fn prompt(&self) -> Option<String> {
                    match self {
                        Self::Inner(_0) => _0.prompt(),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn from_generates_from_impl() {
        let actual = codegen_from_input(quote! {
            enum Outer {
                #[outcome(transparent)]
                Inner(#[from] InnerOutcome),
            }
        });

        // Should contain both Reportable and From impls.
        assert!(actual.contains("impl From<InnerOutcome> for Outer"));
        assert!(actual.contains("Outer::Inner(value)"));
    }

    #[test]
    fn multiple_from_impls() {
        let actual = codegen_from_input(quote! {
            enum Parent {
                #[outcome(transparent)]
                A(#[from] ChildA),
                #[outcome(transparent)]
                B(#[from] ChildB),
            }
        });

        assert!(actual.contains("impl From<ChildA> for Parent"));
        assert!(actual.contains("impl From<ChildB> for Parent"));
    }

    #[test]
    fn message_with_dot_prefix_args() {
        // User writes .0.display() — codegen rewrites to _0.display()
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("path: {}", .0.display()))]
                Found(std::path::PathBuf),
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Found(_0) => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Found(_0) => format!("path: {}", _0.display()),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn format_string_rewrites_positional_with_format_spec() {
        // {0:?} should become {_0:?}
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("debug: {0:?}"))]
                Debug(String),
            }
        });

        assert!(actual.contains(r#"format!("debug: {_0:?}")"#));
    }

    #[test]
    fn escaped_braces_are_preserved() {
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("literal {{braces}}"))]
                Escaped,
            }
        });

        assert!(actual.contains(r#"format!("literal {{braces}}")"#));
    }

    #[test]
    fn dot_prefix_named_field_rewrites() {
        // .field in trailing args should just remove the dot
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("hi {}", .name))]
                Named { name: String },
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::Named { name } => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::Named { name } => format!("hi {}", name),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn log_fallback_to_message_when_no_log() {
        // When some variants have log and others don't, the log_message method
        // should fall back to the message format for variants without log.
        let actual = codegen_from_input(quote! {
            enum TestOutcome {
                #[outcome(message("user msg"), log("internal log"))]
                WithLog,
                #[outcome(message("just message"))]
                WithoutLog,
            }
        });

        let expected = pretty(quote! {
            impl oneiros_outcomes::Reportable for TestOutcome {
                fn level(&self) -> tracing::Level {
                    match self {
                        Self::WithLog => tracing::Level::INFO,
                        Self::WithoutLog => tracing::Level::INFO,
                    }
                }
                fn message(&self) -> String {
                    match self {
                        Self::WithLog => format!("user msg"),
                        Self::WithoutLog => format!("just message"),
                    }
                }
                fn log_message(&self) -> String {
                    match self {
                        Self::WithLog => format!("internal log"),
                        Self::WithoutLog => format!("just message"),
                    }
                }
            }
        });

        pretty_assertions::assert_eq!(actual, expected);
    }
}
