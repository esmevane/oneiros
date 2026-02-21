use data_encoding::BASE64URL_NOPAD;

/// A dual-format identifier that supports both legacy UUIDs and
/// content-addressed links.
///
/// `Legacy` variant wraps a UUID (the original ID format).
/// `Content` variant wraps raw postcard bytes (the content-addressed format).
///
/// Display/FromStr: Legacy renders as UUID string, Content as base64url.
/// FromStr tries UUID parse first, then base64url decode.
/// Serde uses the string representation (Display/FromStr).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Id {
    Legacy(uuid::Uuid),
    Content(bytes::Bytes),
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub fn new() -> Self {
        Self::Legacy(uuid::Uuid::now_v7())
    }

    pub fn content(bytes: &[u8]) -> Self {
        Self::Content(bytes::Bytes::copy_from_slice(bytes))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Legacy(uuid) => uuid.is_nil(),
            Self::Content(bytes) => bytes.is_empty(),
        }
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legacy(uuid) => write!(f, "{uuid}"),
            Self::Content(bytes) => write!(f, "{}", BASE64URL_NOPAD.encode(bytes)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IdParseError {
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    #[error("Unable to parse id, invalid contents: {0}")]
    Invalid(String),
}

impl core::str::FromStr for Id {
    type Err = IdParseError;

    fn from_str(given_str: &str) -> Result<Self, Self::Err> {
        // Try UUID first (most common in existing data).
        if let Ok(uuid) = uuid::Uuid::parse_str(given_str) {
            return Ok(Self::Legacy(uuid));
        }

        // Then try base64url decode for content-addressed IDs.
        if let Ok(bytes) = BASE64URL_NOPAD.decode(given_str.as_bytes())
            && !bytes.is_empty()
        {
            return Ok(Self::content(&bytes));
        }

        Err(IdParseError::Invalid(given_str.to_string()))
    }
}

/// Helper for non-human-readable (binary) serialization formats like postcard.
/// Provides a tagged enum so binary formats can distinguish Legacy from Content.
#[derive(serde::Serialize, serde::Deserialize)]
enum BinaryId {
    Legacy(uuid::Uuid),
    Content(Vec<u8>),
}

impl From<&Id> for BinaryId {
    fn from(id: &Id) -> Self {
        match id {
            Id::Legacy(uuid) => BinaryId::Legacy(*uuid),
            Id::Content(bytes) => BinaryId::Content(bytes.to_vec()),
        }
    }
}

impl From<BinaryId> for Id {
    fn from(binary: BinaryId) -> Self {
        match binary {
            BinaryId::Legacy(uuid) => Id::Legacy(uuid),
            BinaryId::Content(bytes) => Id::Content(bytes::Bytes::from(bytes)),
        }
    }
}

impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            BinaryId::from(self).serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        } else {
            BinaryId::deserialize(deserializer).map(Id::from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_round_trip() {
        let id = Id::new();
        let s = id.to_string();
        let parsed: Id = s.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn content_round_trip() {
        let id = Id::content(&vec![1, 2, 3, 4, 5]);
        let s = id.to_string();
        let parsed: Id = s.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn legacy_serde_round_trip() {
        let id = Id::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: Id = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn content_serde_round_trip() {
        let id = Id::content(&vec![10, 20, 30]);
        let json = serde_json::to_string(&id).unwrap();
        let parsed: Id = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn legacy_and_content_are_never_equal() {
        let legacy = Id::Legacy(uuid::Uuid::nil());
        let content = Id::content(uuid::Uuid::nil().as_bytes());
        assert_ne!(legacy, content);
    }

    #[test]
    fn is_empty() {
        assert!(Id::Legacy(uuid::Uuid::nil()).is_empty());
        assert!(Id::content(&vec![]).is_empty());
        assert!(!Id::new().is_empty());
        assert!(!Id::content(&vec![1]).is_empty());
    }

    #[test]
    fn deterministic_content() {
        let a = Id::content(&vec![1, 2, 3]);
        let b = Id::content(&vec![1, 2, 3]);
        assert_eq!(a, b);
    }

    #[test]
    fn distinct_content() {
        let a = Id::content(&vec![1, 2, 3]);
        let b = Id::content(&vec![4, 5, 6]);
        assert_ne!(a, b);
    }

    #[test]
    fn legacy_postcard_round_trip() {
        let id = Id::new();
        let bytes = postcard::to_allocvec(&id).unwrap();
        let parsed: Id = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn content_postcard_round_trip() {
        let id = Id::content(&vec![10, 20, 30, 40, 50]);
        let bytes = postcard::to_allocvec(&id).unwrap();
        let parsed: Id = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(id, parsed);
    }
}
