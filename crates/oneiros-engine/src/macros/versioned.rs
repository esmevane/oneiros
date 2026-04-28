//! `versioned!` — generate the wrapper-enum boilerplate for a versioned protocol shape.
//!
//! Two call shapes are supported:
//!
//! 1. **struct-inline (default):** struct definitions live inside the macro
//!    call. Struct names are synthesized as `<EnumName><VariantName>` (so
//!    `AgentCreated` + `V1` becomes `AgentCreatedV1`). Each variant accepts
//!    its own attribute slot before the `=>`, where users layer additional
//!    derives (e.g. `clap::Args`, `Hash`) onto a variant's struct. Rust
//!    happily merges multiple `#[derive(...)]` lines, so the macro just
//!    forwards the user's attrs above its own hardcoded baseline.
//!
//!    The hardcoded baseline:
//!    - **Wrapper enum:** `Debug, Clone, Serialize, Deserialize` + user's
//!      enum-level attrs (e.g. `#[derive(JsonSchema)]`).
//!    - **Latest struct:** `Debug, Clone, Builder, Serialize, Deserialize,
//!      JsonSchema` + user's variant-level attrs.
//!    - **Older structs:** `Debug, Clone, Builder, Serialize, Deserialize` +
//!      user's variant-level attrs.
//!
//!    **Args delegation is conditional.** When the latest variant's user
//!    attrs include `#[derive(Args)]` (or `clap::Args`), the macro
//!    additionally emits an `impl clap::Args for Wrapper` that delegates
//!    to `<LatestStruct as clap::Args>`, plus `Wrapper::clap_command(name)`
//!    and `Wrapper::to_invocation(name)` helpers. The detection is by
//!    token-tree match — if `Args` shows up anywhere in the latest variant's
//!    attrs, the delegation is generated.
//!
//! 2. **struct-out (legacy single version):** the V_n struct lives outside
//!    the macro; user names it as a type. Macro generates the wrapper,
//!    `current()`, and `From<V1> for Wrapper`. The user controls all
//!    derives on the V_n struct.
//!
//! # Example — request shape (CLI-visible)
//!
//! ```ignore
//! versioned! {
//!     #[derive(JsonSchema)]
//!     pub enum CreateAgent {
//!         #[derive(Args)]
//!         V1 => {
//!             #[builder(into)] pub name: AgentName,
//!             #[arg(long, default_value = "")]
//!             #[builder(default, into)] pub description: Description,
//!         }
//!     }
//! }
//! ```
//!
//! Generates `CreateAgentV1`, `CreateAgent::current()`,
//! `CreateAgent::builder_v1()`, `From<CreateAgentV1> for CreateAgent`, plus
//! (because `Args` was on the V1 attrs) `clap::Args` impl on `CreateAgent`,
//! `CreateAgent::clap_command(name)`, and `CreateAgent::to_invocation(name)`.
//!
//! # Example — event shape (internal)
//!
//! ```ignore
//! versioned! {
//!     pub enum AgentCreated {
//!         V1 => {
//!             #[builder(default)] pub id: AgentId,
//!             #[builder(into)] pub name: AgentName,
//!         },
//!         V0 => {
//!             #[builder(into)] pub name: AgentName,
//!         },
//!     }
//! }
//!
//! impl TryFrom<AgentCreatedV0> for AgentCreatedV1 { /* user-supplied */ }
//! ```
//!
//! No Args delegation generated, since the V1 attrs don't include it.

/// Internal helper: emit `$body` iff the literal ident `Args` appears
/// anywhere in the supplied token tree haystack. Used by `versioned!` to
/// gate Args delegation generation on whether the user derived
/// `clap::Args` (or `Args`) on the latest variant.
///
/// The implementation is a recursive TT-munching scan: any nested group
/// (parens/brackets/braces) is flattened inline and scanning continues.
/// `Args` is the only ident we look for, so the only false positive
/// would be a non-derive attribute that mentions `Args` as an ident —
/// which doesn't appear in this codebase.
macro_rules! __versioned_if_args {
    // Found the literal ident `Args` — emit body and stop.
    ({ $($body:tt)* }, [Args $($_rest:tt)*]) => { $($body)* };
    // Flatten a paren group and continue.
    ({ $($body:tt)* }, [($($inner:tt)*) $($rest:tt)*]) => {
        $crate::macros::__versioned_if_args!({ $($body)* }, [$($inner)* $($rest)*]);
    };
    // Flatten a bracket group and continue.
    ({ $($body:tt)* }, [[$($inner:tt)*] $($rest:tt)*]) => {
        $crate::macros::__versioned_if_args!({ $($body)* }, [$($inner)* $($rest)*]);
    };
    // Flatten a brace group and continue.
    ({ $($body:tt)* }, [{$($inner:tt)*} $($rest:tt)*]) => {
        $crate::macros::__versioned_if_args!({ $($body)* }, [$($inner)* $($rest)*]);
    };
    // Skip any other token.
    ({ $($body:tt)* }, [$_first:tt $($rest:tt)*]) => {
        $crate::macros::__versioned_if_args!({ $($body)* }, [$($rest)*]);
    };
    // End — Args not found.
    ({ $($body:tt)* }, []) => {};
}

pub(crate) use __versioned_if_args;

macro_rules! versioned {
    // ───────────────────────────────────────────────────────────────────
    // Inline form. Per-variant attribute slot before `=>` lets the user
    // layer arbitrary derives (Args, Hash, custom) onto a variant's
    // struct. Args delegation on the wrapper is conditional on the
    // latest variant's attrs containing `Args`.
    // ───────────────────────────────────────────────────────────────────
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident {
            // Variant attrs are captured as raw token trees (not `:meta`) so
            // the helper macro `__versioned_if_args!` can scan into the
            // `derive(...)` group looking for `Args`. A `:meta` capture
            // flattens group structure when re-emitted into another macro's
            // input, which defeats the scan.
            $(#[$($lattr:tt)*])*
            $latest_variant:ident => {
                $(
                    $(#[$lf_attr:meta])*
                    $lfvis:vis $lf:ident: $lty:ty
                ),* $(,)?
            }
            $(,
                $(#[$($vattr:tt)*])*
                $variant:ident => {
                    $(
                        $(#[$f_attr:meta])*
                        $fvis:vis $f:ident: $fty:ty
                    ),* $(,)?
                }
            )*
            $(,)?
        }
    ) => {
        ::paste::paste! {
            $(#[$enum_attr])*
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
            #[serde(untagged)]
            $vis enum $name {
                $latest_variant([<$name $latest_variant>]),
                $($variant([<$name $variant>]),)*
            }

            $(#[$($lattr)*])*
            #[derive(
                Debug,
                Clone,
                ::bon::Builder,
                ::serde::Serialize,
                ::serde::Deserialize,
                ::schemars::JsonSchema,
            )]
            #[serde(deny_unknown_fields)]
            $vis struct [<$name $latest_variant>] {
                $(
                    $(#[$lf_attr])*
                    $lfvis $lf: $lty,
                )*
            }

            $(
                $(#[$($vattr)*])*
                #[derive(Debug, Clone, ::bon::Builder, ::serde::Serialize, ::serde::Deserialize)]
                #[serde(deny_unknown_fields)]
                $vis struct [<$name $variant>] {
                    $(
                        $(#[$f_attr])*
                        $fvis $f: $fty,
                    )*
                }
            )*

            impl $name {
                pub fn current(
                    &self,
                ) -> ::std::result::Result<[<$name $latest_variant>], $crate::UpcastError> {
                    Ok(match self {
                        Self::$latest_variant(v) => v.clone(),
                        $(Self::$variant(v) => v.clone().try_into()?,)*
                    })
                }

                pub fn [<builder_ $latest_variant:lower>]() -> [<$name $latest_variant Builder>] {
                    [<$name $latest_variant>]::builder()
                }

                $(
                    pub fn [<builder_ $variant:lower>]() -> [<$name $variant Builder>] {
                        [<$name $variant>]::builder()
                    }
                )*
            }

            impl ::std::convert::From<[<$name $latest_variant>]> for $name {
                fn from(v: [<$name $latest_variant>]) -> Self {
                    Self::$latest_variant(v)
                }
            }

            $crate::macros::__versioned_if_args!({
                impl $name {
                    pub fn clap_command(name: &'static str) -> ::clap::Command {
                        <[<$name $latest_variant>] as ::clap::Args>::augment_args(
                            ::clap::Command::new(name),
                        )
                    }

                    pub fn to_invocation(&self, name: &str) -> String {
                        $crate::macros::render_invocation(name, self)
                    }
                }

                impl ::clap::Args for $name {
                    fn augment_args(cmd: ::clap::Command) -> ::clap::Command {
                        <[<$name $latest_variant>] as ::clap::Args>::augment_args(cmd)
                    }
                    fn augment_args_for_update(cmd: ::clap::Command) -> ::clap::Command {
                        <[<$name $latest_variant>] as ::clap::Args>::augment_args_for_update(cmd)
                    }
                }

                impl ::clap::FromArgMatches for $name {
                    fn from_arg_matches(
                        matches: &::clap::ArgMatches,
                    ) -> ::std::result::Result<Self, ::clap::Error> {
                        Ok(Self::$latest_variant(
                            <[<$name $latest_variant>] as ::clap::FromArgMatches>::from_arg_matches(
                                matches,
                            )?,
                        ))
                    }
                    fn update_from_arg_matches(
                        &mut self,
                        matches: &::clap::ArgMatches,
                    ) -> ::std::result::Result<(), ::clap::Error> {
                        *self = <Self as ::clap::FromArgMatches>::from_arg_matches(matches)?;
                        Ok(())
                    }
                }
            }, [$($($lattr)*)*]);
        }
    };

    // ───────────────────────────────────────────────────────────────────
    // Struct-out, single version (legacy form, kept for cases where the
    // V_n struct's derive set must diverge from the inline form).
    // ───────────────────────────────────────────────────────────────────
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $name:ident {
            V1 => $v1_type:ty $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(untagged)]
        $vis enum $name {
            V1($v1_type),
        }

        impl $name {
            pub fn current(&self) -> ::std::result::Result<$v1_type, $crate::UpcastError> {
                Ok(match self {
                    Self::V1(v) => v.clone(),
                })
            }
        }

        impl ::std::convert::From<$v1_type> for $name {
            fn from(v: $v1_type) -> Self {
                Self::V1(v)
            }
        }
    };
}

pub(crate) use versioned;
