use oneiros_model::Id;

#[derive(Debug, thiserror::Error)]
pub enum PrefixError {
    #[error("No match found for prefix '{0}'")]
    NotFound(String),

    #[error("Ambiguous prefix '{0}' — matches {1} IDs")]
    Ambiguous(String, usize),

    #[error("Prefix must be at least 8 hex characters, got: '{0}'")]
    TooShort(String),

    #[error("Prefix contains non-hex characters: '{0}'")]
    InvalidHex(String),
}

#[derive(Clone, Debug)]
pub struct PrefixId(PrefixIdInner);

#[derive(Clone, Debug)]
enum PrefixIdInner {
    Full(Id),
    Prefix(String),
}

impl PrefixId {
    /// If this is a full UUID, return it directly as an `Id`.
    pub fn as_full_id(&self) -> Option<Id> {
        match &self.0 {
            PrefixIdInner::Full(id) => Some(*id),
            PrefixIdInner::Prefix(_) => None,
        }
    }

    /// Resolve this prefix against a set of known IDs.
    /// Full UUIDs pass through directly. Prefixes are matched
    /// against the hex representation (no dashes) of each ID.
    pub fn resolve(&self, known_ids: &[Id]) -> Result<Id, PrefixError> {
        match &self.0 {
            PrefixIdInner::Full(id) => Ok(*id),
            PrefixIdInner::Prefix(prefix) => {
                let matches: Vec<_> = known_ids
                    .iter()
                    .filter(|id| {
                        let hex = id.to_string().replace('-', "");
                        hex.starts_with(prefix.as_str())
                    })
                    .collect();

                match matches.len() {
                    0 => Err(PrefixError::NotFound(prefix.clone())),
                    1 => Ok(*matches[0]),
                    n => Err(PrefixError::Ambiguous(prefix.clone(), n)),
                }
            }
        }
    }
}

impl core::fmt::Display for PrefixId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            PrefixIdInner::Full(id) => write!(f, "{id}"),
            PrefixIdInner::Prefix(prefix) => write!(f, "{prefix}"),
        }
    }
}

impl core::str::FromStr for PrefixId {
    type Err = PrefixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = s.parse::<Id>() {
            return Ok(Self(PrefixIdInner::Full(id)));
        }

        let cleaned = s.replace('-', "");

        if cleaned.len() < 8 {
            return Err(PrefixError::TooShort(s.to_string()));
        }

        if !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(PrefixError::InvalidHex(s.to_string()));
        }

        Ok(Self(PrefixIdInner::Prefix(cleaned)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(s: &str) -> Id {
        s.parse().unwrap()
    }

    #[test]
    fn full_uuid_parses_and_resolves() {
        let uuid_str = "019c5ea2-ba84-7cf1-8113-3db9b418c82c";
        let prefix: PrefixId = uuid_str.parse().unwrap();

        assert!(prefix.as_full_id().is_some());
        assert_eq!(prefix.to_string(), uuid_str);

        let resolved = prefix.resolve(&[]).unwrap();
        assert_eq!(resolved.to_string(), uuid_str);
    }

    #[test]
    fn short_prefix_parses() {
        let prefix: PrefixId = "019c5ea2".parse().unwrap();
        assert!(prefix.as_full_id().is_none());
    }

    #[test]
    fn too_short_prefix_rejected() {
        let result: Result<PrefixId, _> = "019c5e".parse();
        assert!(matches!(result, Err(PrefixError::TooShort(_))));
    }

    #[test]
    fn non_hex_rejected() {
        let result: Result<PrefixId, _> = "019c5eXZ".parse();
        assert!(matches!(result, Err(PrefixError::InvalidHex(_))));
    }

    #[test]
    fn prefix_resolves_unique_match() {
        let id1 = make_id("019c5ea2-ba84-7cf1-8113-3db9b418c82c");
        let id2 = make_id("019c5ea7-0000-7000-8000-000000000000");
        let known = vec![id1, id2];

        let prefix: PrefixId = "019c5ea2".parse().unwrap();
        let resolved = prefix.resolve(&known).unwrap();
        assert_eq!(resolved, id1);
    }

    #[test]
    fn prefix_errors_on_ambiguous() {
        let id1 = make_id("019c5ea2-ba84-7cf1-8113-3db9b418c82c");
        let id2 = make_id("019c5ea2-0000-7000-8000-000000000000");
        let known = vec![id1, id2];

        let prefix: PrefixId = "019c5ea2".parse().unwrap();
        let result = prefix.resolve(&known);
        assert!(matches!(result, Err(PrefixError::Ambiguous(_, 2))));
    }

    #[test]
    fn prefix_errors_on_no_match() {
        let id1 = make_id("019c5ea2-ba84-7cf1-8113-3db9b418c82c");
        let known = vec![id1];

        let prefix: PrefixId = "aabbccdd".parse().unwrap();
        let result = prefix.resolve(&known);
        assert!(matches!(result, Err(PrefixError::NotFound(_))));
    }
}
