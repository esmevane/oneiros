/// A unique identifier wrapping a UUID (v7, time-ordered).
///
/// Display/FromStr use the standard UUID string format.
/// Serde uses the string representation for human-readable formats
/// and raw bytes for binary formats (e.g. postcard).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Id(pub uuid::Uuid);

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_nil()
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct IdParseError(#[from] pub uuid::Error);

impl core::str::FromStr for Id {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let id = Id::new();
        let s = id.to_string();
        let parsed: Id = s.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn serde_round_trip() {
        let id = Id::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: Id = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn is_empty() {
        assert!(Id(uuid::Uuid::nil()).is_empty());
        assert!(!Id::new().is_empty());
    }

    #[test]
    fn postcard_round_trip() {
        let id = Id::new();
        let bytes = postcard::to_allocvec(&id).unwrap();
        let parsed: Id = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(id, parsed);
    }
}
