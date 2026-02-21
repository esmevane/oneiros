use core::fmt;
use core::str::FromStr;

use crate::KeyParseError;

/// A unified identity reference that holds an ID, a Link, or both.
///
/// `Key<I, L>` collapses multiple mechanisms for entity reference into one
/// type with two axes: what you *have* (Id, Link, or Both) and what you
/// *know* about the type (erased via `Key<Id, Link>` or specialized via
/// `Key<AgentId, AgentLink>`).
///
/// # Resolution
///
/// A Key starts with whatever identity information is available and can
/// be upgraded via `with_id` or `with_link` to carry both forms. The
/// `Both` variant is the fully-resolved state.
///
/// # Broadening and Narrowing
///
/// Typed keys (e.g., `Key<AgentId, AgentLink>`) can be broadened to
/// erased keys (`Key<Id, Link>`) infallibly via `broaden()`. Erased keys
/// can be narrowed to typed keys fallibly via `narrow()` — the Link side
/// carries a resource label that is checked during narrowing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key<I, L> {
    Id(I),
    Link(L),
    Both(I, L),
}

impl<I, L> Key<I, L> {
    /// Returns the ID if present.
    pub fn try_id(&self) -> Option<&I> {
        match self {
            Key::Id(id) | Key::Both(id, _) => Some(id),
            Key::Link(_) => None,
        }
    }

    /// Returns the Link if present.
    pub fn try_link(&self) -> Option<&L> {
        match self {
            Key::Link(link) | Key::Both(_, link) => Some(link),
            Key::Id(_) => None,
        }
    }

    /// Returns the ID or calls `err` to produce an error.
    pub fn id_or<E>(&self, err: impl FnOnce() -> E) -> Result<&I, E> {
        self.try_id().ok_or_else(err)
    }

    /// Returns the Link or calls `err` to produce an error.
    pub fn link_or<E>(&self, err: impl FnOnce() -> E) -> Result<&L, E> {
        self.try_link().ok_or_else(err)
    }

    /// Upgrade to `Both` by adding an ID. If already `Both`, replaces the ID.
    pub fn with_id(self, id: I) -> Self {
        match self {
            Key::Id(_) | Key::Both(_, _) => {
                if let Some(link) = self.into_link() {
                    Key::Both(id, link)
                } else {
                    Key::Id(id)
                }
            }
            Key::Link(link) => Key::Both(id, link),
        }
    }

    /// Upgrade to `Both` by adding a Link. If already `Both`, replaces the Link.
    pub fn with_link(self, link: L) -> Self {
        match self {
            Key::Link(_) | Key::Both(_, _) => {
                if let Some(id) = self.into_id() {
                    Key::Both(id, link)
                } else {
                    Key::Link(link)
                }
            }
            Key::Id(id) => Key::Both(id, link),
        }
    }

    /// Consume and return the ID if present.
    pub fn into_id(self) -> Option<I> {
        match self {
            Key::Id(id) | Key::Both(id, _) => Some(id),
            Key::Link(_) => None,
        }
    }

    /// Consume and return the Link if present.
    pub fn into_link(self) -> Option<L> {
        match self {
            Key::Link(link) | Key::Both(_, link) => Some(link),
            Key::Id(_) => None,
        }
    }

    /// Broaden a typed key to an erased key.
    ///
    /// Both the ID and Link sides are converted via `Into`, which is
    /// infallible for domain types broadening to their generic forms
    /// (e.g., `AgentId → Id`, `AgentLink → Link`).
    pub fn broaden<EI, EL>(self) -> Key<EI, EL>
    where
        I: Into<EI>,
        L: Into<EL>,
    {
        match self {
            Key::Id(id) => Key::Id(id.into()),
            Key::Link(link) => Key::Link(link.into()),
            Key::Both(id, link) => Key::Both(id.into(), link.into()),
        }
    }

    /// Narrow an erased key to a typed key.
    ///
    /// The ID side narrows infallibly (just wrapping). The Link side
    /// narrows fallibly — the link's resource label is checked against
    /// the expected type. On failure, returns the narrowing error from
    /// the Link conversion.
    pub fn narrow<NI, NL, E>(self) -> Result<Key<NI, NL>, E>
    where
        NI: From<I>,
        NL: TryFrom<L, Error = E>,
    {
        match self {
            Key::Id(id) => Ok(Key::Id(NI::from(id))),
            Key::Link(link) => Ok(Key::Link(NL::try_from(link)?)),
            Key::Both(id, link) => Ok(Key::Both(NI::from(id), NL::try_from(link)?)),
        }
    }

    /// Map the ID side of the key.
    pub fn map_id<NI>(self, f: impl FnOnce(I) -> NI) -> Key<NI, L> {
        match self {
            Key::Id(id) => Key::Id(f(id)),
            Key::Link(link) => Key::Link(link),
            Key::Both(id, link) => Key::Both(f(id), link),
        }
    }

    /// Map the Link side of the key.
    pub fn map_link<NL>(self, f: impl FnOnce(L) -> NL) -> Key<I, NL> {
        match self {
            Key::Id(id) => Key::Id(id),
            Key::Link(link) => Key::Link(f(link)),
            Key::Both(id, link) => Key::Both(id, f(link)),
        }
    }
}

impl<I: fmt::Display, L: fmt::Display> fmt::Display for Key<I, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Id(id) => write!(f, "{id}"),
            Key::Link(link) => write!(f, "{link}"),
            Key::Both(id, _) => write!(f, "{id}"),
        }
    }
}

impl<I: FromStr, L: FromStr> FromStr for Key<I, L> {
    type Err = KeyParseError;

    /// Parse a string as a Key by trying the Link side first (more restrictive),
    /// then falling back to the Id side.
    ///
    /// This ordering works because typed links (base64url + label check) have a
    /// very specific format that won't match UUIDs or plain names, while name
    /// types accept any string as a fallback.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(link) = s.parse::<L>() {
            return Ok(Key::Link(link));
        }

        if let Ok(id) = s.parse::<I>() {
            return Ok(Key::Id(id));
        }

        Err(KeyParseError {
            input: s.to_owned(),
        })
    }
}

impl<'de, I: FromStr, L: FromStr> serde::Deserialize<'de> for Key<I, L> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_variant_accessors() {
        let key: Key<u64, String> = Key::Id(42);
        assert_eq!(key.try_id(), Some(&42));
        assert_eq!(key.try_link(), None);
    }

    #[test]
    fn link_variant_accessors() {
        let key: Key<u64, String> = Key::Link("abc".into());
        assert_eq!(key.try_id(), None);
        assert_eq!(key.try_link(), Some(&"abc".into()));
    }

    #[test]
    fn both_variant_accessors() {
        let key: Key<u64, String> = Key::Both(42, "abc".into());
        assert_eq!(key.try_id(), Some(&42));
        assert_eq!(key.try_link(), Some(&"abc".into()));
    }

    #[test]
    fn id_or_returns_id_when_present() {
        let key: Key<u64, String> = Key::Id(42);
        assert_eq!(key.id_or(|| "missing"), Ok(&42));
    }

    #[test]
    fn id_or_returns_error_when_absent() {
        let key: Key<u64, String> = Key::Link("abc".into());
        assert_eq!(key.id_or(|| "missing"), Err("missing"));
    }

    #[test]
    fn link_or_returns_link_when_present() {
        let key: Key<u64, String> = Key::Link("abc".into());
        assert_eq!(key.link_or(|| "missing"), Ok(&String::from("abc")));
    }

    #[test]
    fn link_or_returns_error_when_absent() {
        let key: Key<u64, String> = Key::Id(42);
        assert_eq!(key.link_or(|| "missing"), Err("missing"));
    }

    #[test]
    fn with_id_upgrades_link_to_both() {
        let key: Key<u64, String> = Key::Link("abc".into());
        let upgraded = key.with_id(42);
        assert_eq!(upgraded, Key::Both(42, "abc".into()));
    }

    #[test]
    fn with_link_upgrades_id_to_both() {
        let key: Key<u64, String> = Key::Id(42);
        let upgraded = key.with_link("abc".into());
        assert_eq!(upgraded, Key::Both(42, "abc".into()));
    }

    #[test]
    fn display_prefers_id() {
        let key: Key<u64, String> = Key::Both(42, "abc".into());
        assert_eq!(key.to_string(), "42");
    }

    #[test]
    fn display_shows_link_when_no_id() {
        let key: Key<u64, String> = Key::Link("abc".into());
        assert_eq!(key.to_string(), "abc");
    }

    #[test]
    fn broaden_converts_both_sides() {
        // u32 → u64 broadening
        let key: Key<u32, u32> = Key::Both(1, 2);
        let broad: Key<u64, u64> = key.broaden();
        assert_eq!(broad, Key::Both(1u64, 2u64));
    }

    #[test]
    fn narrow_succeeds_when_link_matches() {
        // Narrowing where both sides succeed
        let key: Key<u64, u64> = Key::Both(42, 100);
        let narrow: Result<Key<u64, u64>, std::convert::Infallible> = key.narrow();
        assert!(narrow.is_ok());
    }

    #[test]
    fn map_id_transforms_id_side() {
        let key: Key<u32, String> = Key::Both(42, "abc".into());
        let mapped = key.map_id(|n| n as u64);
        assert_eq!(mapped, Key::Both(42u64, "abc".into()));
    }

    #[test]
    fn map_link_transforms_link_side() {
        let key: Key<u32, String> = Key::Both(42, "abc".into());
        let mapped = key.map_link(|s| s.len());
        assert_eq!(mapped, Key::Both(42, 3));
    }

    #[test]
    fn into_id_consumes_and_returns() {
        let key: Key<u64, String> = Key::Both(42, "abc".into());
        assert_eq!(key.into_id(), Some(42));
    }

    #[test]
    fn into_link_consumes_and_returns() {
        let key: Key<u64, String> = Key::Both(42, "abc".into());
        assert_eq!(key.into_link(), Some("abc".into()));
    }

    #[test]
    fn into_id_returns_none_for_link_variant() {
        let key: Key<u64, String> = Key::Link("abc".into());
        assert_eq!(key.into_id(), None);
    }

    #[test]
    fn into_link_returns_none_for_id_variant() {
        let key: Key<u64, String> = Key::Id(42);
        assert_eq!(key.into_link(), None);
    }

    // FromStr tests — Key<Id, Link> where Link is more restrictive than Id.
    // Using Key<String, u64>: u64 only parses numbers (simulates restrictive Link),
    // String accepts anything (simulates permissive Name/Id).

    #[test]
    fn from_str_prefers_link_when_both_match() {
        // "42" is valid as both u64 (link) and String (id).
        // Link is tried first, so we get Key::Link.
        let key: Key<String, u64> = "42".parse().unwrap();
        assert_eq!(key, Key::Link(42));
    }

    #[test]
    fn from_str_falls_back_to_id() {
        // "hello" is only valid as String (id), not u64 (link).
        let key: Key<String, u64> = "hello".parse().unwrap();
        assert_eq!(key, Key::Id("hello".into()));
    }

    #[test]
    fn from_str_fails_when_neither_matches() {
        // Both u64 (id) and u64 (link) require numeric strings.
        let result: Result<Key<u64, u64>, _> = "hello".parse();
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_from_string() {
        // Serde deserialization delegates to FromStr.
        let key: Key<String, u64> = serde_json::from_value(serde_json::json!("hello")).unwrap();
        assert_eq!(key, Key::Id("hello".into()));

        let key: Key<String, u64> = serde_json::from_value(serde_json::json!("42")).unwrap();
        assert_eq!(key, Key::Link(42));
    }
}
