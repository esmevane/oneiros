/// Creates a typed domain link wrapper around [`Link`](crate::Link).
///
/// Parallel to `domain_id!` in oneiros-model. Each typed link carries a
/// compile-time label that matches the entity's `address_label()`. Narrowing
/// from a generic `Link` into a typed link checks the label prefix; broadening
/// from a typed link to `Link` is infallible.
///
/// # Usage
///
/// ```ignore
/// oneiros_link::domain_link!(AgentLink, "agent");
/// ```
///
/// Generates:
/// - `pub struct AgentLink(Link)` with transparent serde
/// - `From<AgentLink> for Link` (broadening)
/// - `TryFrom<Link> for AgentLink` (narrowing with label check)
/// - `Display`, `FromStr`, `AsRef<Link>`
#[macro_export]
macro_rules! domain_link {
    ($name:ident, $label:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name($crate::Link);

        impl $name {
            /// The address label this typed link expects.
            pub const LABEL: &'static str = $label;
        }

        impl From<$name> for $crate::Link {
            fn from(typed: $name) -> Self {
                typed.0
            }
        }

        impl TryFrom<$crate::Link> for $name {
            type Error = $crate::LinkNarrowingError;

            fn try_from(link: $crate::Link) -> Result<Self, Self::Error> {
                if link.has_label($label) {
                    Ok(Self(link))
                } else {
                    Err($crate::LinkNarrowingError {
                        expected: $label,
                        link,
                    })
                }
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl core::str::FromStr for $name {
            type Err = $crate::LinkError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let link: $crate::Link = s.parse()?;
                Self::try_from(link).map_err(Into::into)
            }
        }

        impl AsRef<$crate::Link> for $name {
            fn as_ref(&self) -> &$crate::Link {
                &self.0
            }
        }
    };
}
