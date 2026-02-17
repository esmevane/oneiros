use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Id {
    Legacy(uuid::Uuid),
    Content([u8; 32]),
}

impl Id {
    pub fn new() -> Self {
        Self::Legacy(uuid::Uuid::now_v7())
    }

    pub fn from_content(bytes: &[u8]) -> Self {
        let hash = Sha256::digest(bytes);
        Self::Content(hash.into())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Legacy(uuid) => uuid.is_nil(),
            Self::Content(hash) => hash.iter().all(|&b| b == 0),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legacy(uuid) => write!(f, "{uuid}"),
            Self::Content(hash) => {
                for byte in hash {
                    write!(f, "{byte:02x}")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IdParseError {
    #[error("Unable to parse id as UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Unable to parse id as content hash: {0}")]
    InvalidHex(String),
}

impl core::str::FromStr for Id {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try UUID first (36 chars with hyphens, or 32 hex without).
        if let Ok(uuid) = uuid::Uuid::parse_str(s) {
            return Ok(Self::Legacy(uuid));
        }

        // Try 64-char lowercase hex as content hash.
        if s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            let mut hash = [0u8; 32];
            for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
                let hex_str = core::str::from_utf8(chunk)
                    .map_err(|e| IdParseError::InvalidHex(e.to_string()))?;
                hash[i] = u8::from_str_radix(hex_str, 16)
                    .map_err(|e| IdParseError::InvalidHex(e.to_string()))?;
            }
            return Ok(Self::Content(hash));
        }

        // Neither format matched — report as UUID error for backward compat.
        Err(IdParseError::InvalidUuid(
            uuid::Uuid::parse_str(s).unwrap_err(),
        ))
    }
}

impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_display_fromstr_roundtrip() {
        let id = Id::new();
        let s = id.to_string();
        let parsed: Id = s.parse().unwrap();
        assert_eq!(id, parsed);
        // UUID format: 8-4-4-4-12
        assert_eq!(s.len(), 36);
        assert_eq!(s.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn content_display_fromstr_roundtrip() {
        let id = Id::from_content(b"hello world");
        let s = id.to_string();
        let parsed: Id = s.parse().unwrap();
        assert_eq!(id, parsed);
        // Content hash: 64 hex chars, no hyphens
        assert_eq!(s.len(), 64);
        assert!(s.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn from_content_is_deterministic() {
        let a = Id::from_content(b"same input");
        let b = Id::from_content(b"same input");
        assert_eq!(a, b);
    }

    #[test]
    fn from_content_is_distinct() {
        let a = Id::from_content(b"input one");
        let b = Id::from_content(b"input two");
        assert_ne!(a, b);
    }

    #[test]
    fn serde_roundtrip_legacy() {
        let id = Id::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: Id = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
        // JSON is a quoted string
        assert!(json.starts_with('"'));
        assert!(json.ends_with('"'));
    }

    #[test]
    fn serde_roundtrip_content() {
        let id = Id::from_content(b"test data");
        let json = serde_json::to_string(&id).unwrap();
        let parsed: Id = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn is_empty_legacy() {
        let nil = Id::Legacy(uuid::Uuid::nil());
        assert!(nil.is_empty());
        let non_nil = Id::new();
        assert!(!non_nil.is_empty());
    }

    #[test]
    fn is_empty_content() {
        let zero = Id::Content([0u8; 32]);
        assert!(zero.is_empty());
        let non_zero = Id::from_content(b"not empty");
        assert!(!non_zero.is_empty());
    }

    #[test]
    fn legacy_and_content_are_never_equal() {
        // Even if they happen to share some bytes, different variants are different.
        let legacy = Id::new();
        let content = Id::from_content(legacy.to_string().as_bytes());
        assert_ne!(legacy, content);
    }

    #[test]
    fn parse_known_uuid() {
        let id: Id = "019c5ea2-ba84-7cf1-8113-3db9b418c82c".parse().unwrap();
        assert!(matches!(id, Id::Legacy(_)));
    }

    #[test]
    fn parse_known_content_hash() {
        // SHA-256 of empty string
        let hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let id: Id = hash.parse().unwrap();
        assert!(matches!(id, Id::Content(_)));
        assert_eq!(id.to_string(), hash);
    }
}
