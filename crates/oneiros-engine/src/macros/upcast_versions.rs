//! `upcast_versions!` — generate `From` / `TryFrom` impls between versioned shapes.
//!
//! The macro destructures the source by the LHS field list and splices your
//! body verbatim into `Self { ... }`. Nothing is hidden — every field that
//! ends up in the target appears in the body explicitly. Field shorthand
//! picks up bindings from the destructure when you reference them by name.
//!
//! A single invocation can declare any number of pairs; mix bare and
//! `Ok`-wrapped bodies freely.
//!
//! # Bare body — generates `From<Source> for Target`
//!
//! ```ignore
//! upcast_versions! {
//!     AgentCreatedV0 { name, description, prompt } => AgentCreatedV1 {
//!         name, description, prompt,
//!         id: AgentId::new(),
//!         persona: PersonaName::legacy(),
//!     }
//! }
//! ```
//!
//! Because `From<T> for U` blanket-implies `TryFrom<T> for U` with
//! `Error = Infallible`, callers using `try_into()?` work unchanged.
//!
//! # Ok-wrapped body — generates `TryFrom<Source> for Target` with `UpcastError`
//!
//! ```ignore
//! upcast_versions! {
//!     AgentCreatedV0 { name, description, prompt } => Ok(AgentCreatedV1 {
//!         name, description, prompt,
//!         id: AgentId::new(),
//!         persona: PersonaName::legacy(),
//!     })
//! }
//! ```
//!
//! # Chained — many pairs in one call
//!
//! ```ignore
//! upcast_versions! {
//!     V0 { name, value } => V1 {
//!         name, value,
//!         id: 42,
//!     }
//!     V1 { name, value, id } => Ok(V2 {
//!         name, value, id,
//!         timestamp: Timestamp::now(),
//!     })
//! }
//! ```
//!
//! Each pair becomes its own `impl`. `versioned!`'s generated `current()`
//! walks the chain via `try_into()?` at the call site.
//!
//! # When to reach for this macro vs. a hand-written `From`
//!
//! `upcast_versions!` is for *version evolution*. Plain structural
//! conversions — entity ↔ event payload, response envelope reshaping,
//! anything where there's no version timeline — are not upcasts; hand-write
//! the `From` impls for those, even if the field-shuffle looks identical.
//! The semantic distinction is worth the small redundancy.
//!
//! # Discontinuity
//!
//! Pure-discontinuity upcasts (always `Err`) live under
//! [`unsupported_upcast!`](crate::unsupported_upcast).
macro_rules! upcast_versions {
    // Bare pair → From, then recurse on the rest.
    (
        $source:ident { $($pat:tt)* } => $target:ident { $($body:tt)* }
        $($rest:tt)*
    ) => {
        impl ::std::convert::From<$source> for $target {
            fn from(previous: $source) -> Self {
                let $source { $($pat)* } = previous;
                Self { $($body)* }
            }
        }
        upcast_versions! { $($rest)* }
    };

    // Ok-wrapped pair → TryFrom with UpcastError, then recurse on the rest.
    (
        $source:ident { $($pat:tt)* } => Ok($target:ident { $($body:tt)* })
        $($rest:tt)*
    ) => {
        impl ::std::convert::TryFrom<$source> for $target {
            type Error = $crate::UpcastError;
            fn try_from(previous: $source) -> ::std::result::Result<Self, Self::Error> {
                let $source { $($pat)* } = previous;
                Ok(Self { $($body)* })
            }
        }
        upcast_versions! { $($rest)* }
    };

    // Empty — terminate the recursion.
    () => {};
}

pub(crate) use upcast_versions;

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, PartialEq)]
    struct Wedge0 {
        name: String,
        value: u32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Wedge1 {
        name: String,
        value: u32,
        id: u64,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Wedge2 {
        name: String,
        value: u32,
        id: u64,
        timestamp: u64,
    }

    upcast_versions! {
        Wedge0 { name, value } => Wedge1 {
            name, value,
            id: 42,
        }
        Wedge1 { name, value, id } => Ok(Wedge2 {
            name, value, id,
            timestamp: 100,
        })
    }

    #[test]
    fn bare_form_emits_from_impl() {
        let v0 = Wedge0 {
            name: "foo".into(),
            value: 5,
        };
        let v1: Wedge1 = v0.into();
        assert_eq!(v1.name, "foo");
        assert_eq!(v1.value, 5);
        assert_eq!(v1.id, 42);
    }

    #[test]
    fn bare_form_blanket_lets_try_into_work() {
        let v0 = Wedge0 {
            name: "foo".into(),
            value: 5,
        };
        #[allow(clippy::unnecessary_fallible_conversions)]
        let v1: Wedge1 = v0.try_into().unwrap();
        assert_eq!(v1.id, 42);
    }

    #[test]
    fn ok_form_emits_try_from_impl() {
        let v1 = Wedge1 {
            name: "foo".into(),
            value: 5,
            id: 7,
        };
        let v2 = Wedge2::try_from(v1).unwrap();
        assert_eq!(v2.name, "foo");
        assert_eq!(v2.value, 5);
        assert_eq!(v2.id, 7);
        assert_eq!(v2.timestamp, 100);
    }

    #[test]
    fn chains_via_try_into_through_both_impls() {
        let v0 = Wedge0 {
            name: "foo".into(),
            value: 5,
        };
        #[allow(clippy::unnecessary_fallible_conversions)]
        let v1: Wedge1 = v0.try_into().unwrap();
        let v2: Wedge2 = v1.try_into().unwrap();
        assert_eq!(v2.name, "foo");
        assert_eq!(v2.id, 42);
        assert_eq!(v2.timestamp, 100);
    }
}
