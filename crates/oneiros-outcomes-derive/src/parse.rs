use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Fields, Ident, LitStr, Token, Type};

/// The parsed domain model for an entire `#[derive(Outcome)]` enum.
#[derive(Debug)]
pub struct OutcomeModel {
    pub ident: Ident,
    pub generics: syn::Generics,
    pub variants: Vec<VariantModel>,
}

/// A single enum variant with its parsed attributes and fields.
#[derive(Debug)]
pub struct VariantModel {
    pub ident: Ident,
    pub fields: Vec<FieldModel>,
    pub config: OutcomeConfig,
}

/// A single field within a variant.
#[derive(Debug)]
pub struct FieldModel {
    pub member: syn::Member,
    pub ty: Type,
    pub is_from: bool,
}

/// The `#[outcome(...)]` configuration for a variant.
#[derive(Debug)]
pub enum OutcomeConfig {
    Transparent {
        span: Span,
    },
    Explicit {
        message: FormatArgs,
        level: Level,
        log: Option<FormatArgs>,
        prompt: Option<FormatArgs>,
    },
}

/// A parsed format string with optional trailing arguments, e.g. `message("foo {}", bar.baz())`.
#[derive(Debug)]
pub struct FormatArgs {
    pub fmt: LitStr,
    pub args: Vec<proc_macro2::TokenStream>,
}

/// A parsed log level, defaulting to Info.
#[derive(Clone, Copy, Debug, Default)]
pub enum Level {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Level {
    fn from_str(s: &str, span: Span) -> syn::Result<Self> {
        match s {
            "trace" => Ok(Level::Trace),
            "debug" => Ok(Level::Debug),
            "info" => Ok(Level::Info),
            "warn" => Ok(Level::Warn),
            "error" => Ok(Level::Error),
            _ => Err(syn::Error::new(
                span,
                format!("invalid level \"{s}\", expected one of: trace, debug, info, warn, error"),
            )),
        }
    }
}

/// Parse a `DeriveInput` into our domain model.
pub fn parse(input: &DeriveInput) -> syn::Result<OutcomeModel> {
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "Outcome can only be derived for enums",
            ));
        }
    };

    let variants = data
        .variants
        .iter()
        .map(parse_variant)
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(OutcomeModel {
        ident: input.ident.clone(),
        generics: input.generics.clone(),
        variants,
    })
}

fn parse_variant(variant: &syn::Variant) -> syn::Result<VariantModel> {
    let config = parse_outcome_attr(&variant.attrs, &variant.ident)?;
    let fields = parse_fields(&variant.fields)?;

    if let OutcomeConfig::Transparent { span } = &config
        && fields.len() != 1
    {
        return Err(syn::Error::new(
            *span,
            "transparent outcome requires exactly one field",
        ));
    }

    Ok(VariantModel {
        ident: variant.ident.clone(),
        fields,
        config,
    })
}

fn parse_fields(fields: &Fields) -> syn::Result<Vec<FieldModel>> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                let is_from = has_from_attr(&f.attrs);
                Ok(FieldModel {
                    member: syn::Member::Named(f.ident.clone().unwrap()),
                    ty: f.ty.clone(),
                    is_from,
                })
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let is_from = has_from_attr(&f.attrs);
                Ok(FieldModel {
                    member: syn::Member::Unnamed(syn::Index {
                        index: i as u32,
                        span: f.span(),
                    }),
                    ty: f.ty.clone(),
                    is_from,
                })
            })
            .collect(),
        Fields::Unit => Ok(Vec::new()),
    }
}

fn has_from_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("from"))
}

/// Parse the `#[outcome(...)]` attribute from a variant's attributes.
fn parse_outcome_attr(attrs: &[Attribute], variant_ident: &Ident) -> syn::Result<OutcomeConfig> {
    let outcome_attr = attrs
        .iter()
        .find(|a| a.path().is_ident("outcome"))
        .ok_or_else(|| {
            syn::Error::new(
                variant_ident.span(),
                format!("missing #[outcome(...)] attribute on variant `{variant_ident}`"),
            )
        })?;

    outcome_attr.parse_args_with(parse_outcome_config)
}

/// The inner parser for `#[outcome(...)]` content.
fn parse_outcome_config(input: ParseStream) -> syn::Result<OutcomeConfig> {
    // Check for `transparent` keyword first.
    if input.peek(syn::Ident) {
        let ident: Ident = input.fork().parse()?;
        if ident == "transparent" {
            let ident: Ident = input.parse()?;
            if !input.is_empty() {
                return Err(syn::Error::new(
                    input.span(),
                    "unexpected tokens after `transparent`",
                ));
            }
            return Ok(OutcomeConfig::Transparent { span: ident.span() });
        }
    }

    let mut message: Option<FormatArgs> = None;
    let mut level: Option<Level> = None;
    let mut log: Option<FormatArgs> = None;
    let mut prompt: Option<FormatArgs> = None;

    while !input.is_empty() {
        let key: Ident = input.parse()?;
        match key.to_string().as_str() {
            "message" => {
                let content;
                syn::parenthesized!(content in input);
                message = Some(parse_format_args(&content)?);
            }
            "log" => {
                let content;
                syn::parenthesized!(content in input);
                log = Some(parse_format_args(&content)?);
            }
            "prompt" => {
                let content;
                syn::parenthesized!(content in input);
                prompt = Some(parse_format_args(&content)?);
            }
            "level" => {
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                level = Some(Level::from_str(&lit.value(), lit.span())?);
            }
            other => {
                return Err(syn::Error::new(
                    key.span(),
                    format!("unknown outcome attribute `{other}`"),
                ));
            }
        }

        // Consume optional trailing comma.
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
    }

    let message = message.ok_or_else(|| {
        syn::Error::new(
            input.span(),
            "missing required `message(\"...\")` in #[outcome(...)]",
        )
    })?;

    Ok(OutcomeConfig::Explicit {
        message,
        level: level.unwrap_or_default(),
        log,
        prompt,
    })
}

/// Parse `"format string", arg1, arg2, ...` within parentheses.
/// Supports thiserror-style dot-prefix syntax: `.0` becomes `_0`, `.field` becomes `field`.
fn parse_format_args(input: ParseStream) -> syn::Result<FormatArgs> {
    let fmt: LitStr = input.parse()?;
    let mut args = Vec::new();

    while input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            break;
        }

        // Check for dot-prefix syntax (`.0`, `.field`)
        if input.peek(Token![.]) {
            let dot_span = input.parse::<Token![.]>()?.span;

            let prefix = if input.peek(syn::LitInt) {
                // `.0` → `_0`
                let lit: syn::LitInt = input.parse()?;
                quote::format_ident!("_{}", lit.to_string(), span = lit.span())
            } else if input.peek(syn::Ident) {
                // `.field` → `field`
                input.parse::<Ident>()?
            } else {
                return Err(syn::Error::new(
                    dot_span,
                    "expected field index or name after `.`",
                ));
            };

            // Collect remaining tokens until comma or end to form the
            // full expression (handles method chains, closures, turbofish, etc.)
            let mut rest_tokens = proc_macro2::TokenStream::new();
            while !input.is_empty() && !input.peek(Token![,]) {
                let tt: proc_macro2::TokenTree = input.parse()?;
                rest_tokens.extend(std::iter::once(tt));
            }
            args.push(quote::quote!(#prefix #rest_tokens));
        } else {
            // Normal expression (includes `name = expr` for named format args)
            let expr: syn::Expr = input.parse()?;
            args.push(quote::quote!(#expr));
        }
    }

    Ok(FormatArgs { fmt, args })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input(tokens: proc_macro2::TokenStream) -> syn::Result<OutcomeModel> {
        let input: DeriveInput = syn::parse2(tokens)?;
        parse(&input)
    }

    #[test]
    fn parses_simple_message() {
        let model = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(message("hello world"))]
                Simple,
            }
        })
        .unwrap();

        assert_eq!(model.ident, "TestOutcome");
        assert_eq!(model.variants.len(), 1);
        assert_eq!(model.variants[0].ident, "Simple");
        assert!(model.variants[0].fields.is_empty());

        match &model.variants[0].config {
            OutcomeConfig::Explicit { message, level, .. } => {
                assert_eq!(message.fmt.value(), "hello world");
                assert!(message.args.is_empty());
                assert!(matches!(level, Level::Info));
            }
            _ => panic!("expected Explicit config"),
        }
    }

    #[test]
    fn parses_message_with_level() {
        let model = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(message("warning!"), level = "warn")]
                Bad,
            }
        })
        .unwrap();

        match &model.variants[0].config {
            OutcomeConfig::Explicit { level, .. } => {
                assert!(matches!(level, Level::Warn));
            }
            _ => panic!("expected Explicit config"),
        }
    }

    #[test]
    fn parses_message_with_format_args() {
        let model = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(message("hello {}", .0.display()))]
                WithArgs(String),
            }
        })
        .unwrap();

        match &model.variants[0].config {
            OutcomeConfig::Explicit { message, .. } => {
                assert_eq!(message.fmt.value(), "hello {}");
                assert_eq!(message.args.len(), 1);
                // .0.display() should have been rewritten to _0.display()
                let arg_str = message.args[0].to_string();
                assert!(arg_str.contains("_0"), "expected _0 in arg, got: {arg_str}");
            }
            _ => panic!("expected Explicit config"),
        }
    }

    #[test]
    fn parses_all_attributes() {
        let model = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(
                    message("user msg {0}"),
                    log("log msg {0}"),
                    prompt("do this next"),
                    level = "debug"
                )]
                Full(String),
            }
        })
        .unwrap();

        match &model.variants[0].config {
            OutcomeConfig::Explicit {
                message,
                level,
                log,
                prompt,
            } => {
                assert_eq!(message.fmt.value(), "user msg {0}");
                assert!(matches!(level, Level::Debug));
                assert_eq!(log.as_ref().unwrap().fmt.value(), "log msg {0}");
                assert_eq!(prompt.as_ref().unwrap().fmt.value(), "do this next");
            }
            _ => panic!("expected Explicit config"),
        }
    }

    #[test]
    fn parses_transparent() {
        let model = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(transparent)]
                Inner(InnerOutcome),
            }
        })
        .unwrap();

        assert!(matches!(
            model.variants[0].config,
            OutcomeConfig::Transparent { .. }
        ));
    }

    #[test]
    fn parses_from_on_field() {
        let model = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(transparent)]
                Inner(#[from] InnerOutcome),
            }
        })
        .unwrap();

        assert!(model.variants[0].fields[0].is_from);
    }

    #[test]
    fn rejects_missing_outcome_attr() {
        let result = parse_input(quote::quote! {
            enum TestOutcome {
                NoAttr,
            }
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("missing #[outcome(...)"));
    }

    #[test]
    fn rejects_missing_message() {
        let result = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(level = "warn")]
                NoMessage,
            }
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("missing required `message"));
    }

    #[test]
    fn rejects_invalid_level() {
        let result = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(message("hi"), level = "critical")]
                Bad,
            }
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid level"));
    }

    #[test]
    fn rejects_transparent_with_multiple_fields() {
        let result = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(transparent)]
                Multi(String, String),
            }
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("exactly one field"));
    }

    #[test]
    fn rejects_structs() {
        let result = parse_input(quote::quote! {
            struct NotAnEnum {
                field: String,
            }
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("enums"));
    }

    #[test]
    fn rejects_unknown_attribute() {
        let result = parse_input(quote::quote! {
            enum TestOutcome {
                #[outcome(message("hi"), bogus = "what")]
                Bad,
            }
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("unknown outcome attribute"));
    }
}
