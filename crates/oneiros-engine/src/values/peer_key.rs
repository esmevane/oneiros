use data_encoding::HEXLOWER;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A peer's cryptographic identity — the ed25519 public key of a remote host.
///
/// Stored as a 32-byte array; displayed as hex for human legibility; serialized
/// as a hex string for wire and storage formats. The key is the unforgeable
/// identity of a peer; the system-level `PeerId` is the domain handle used for
/// internal lookup and references.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PeerKey([u8; Self::LENGTH]);

#[derive(Debug, thiserror::Error)]
pub enum PeerKeyError {
    #[error("invalid hex encoding: {0}")]
    Encoding(#[from] data_encoding::DecodeError),
    #[error("invalid peer key length: expected {expected} bytes, got {got}")]
    InvalidLength { expected: usize, got: usize },
}

impl PeerKey {
    pub const LENGTH: usize = 32;

    pub fn from_bytes(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }
}

impl core::fmt::Display for PeerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", HEXLOWER.encode(&self.0))
    }
}

impl core::str::FromStr for PeerKey {
    type Err = PeerKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = HEXLOWER.decode(s.as_bytes())?;
        let len = bytes.len();
        let arr: [u8; Self::LENGTH] =
            bytes.try_into().map_err(|_| PeerKeyError::InvalidLength {
                expected: Self::LENGTH,
                got: len,
            })?;
        Ok(Self(arr))
    }
}

impl Serialize for PeerKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PeerKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl schemars::JsonSchema for PeerKey {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("PeerKey")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "description": "Hex-encoded 32-byte ed25519 public key"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn peer_key_display_is_hex() {
        let key = PeerKey::from_bytes([0xab; 32]);
        let display = key.to_string();
        assert_eq!(display.len(), 64);
        assert!(display.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn peer_key_roundtrip_through_display() {
        let original = PeerKey::from_bytes([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0xde, 0xad, 0xbe, 0xef,
            0xca, 0xfe, 0xba, 0xbe,
        ]);
        let encoded = original.to_string();
        let decoded: PeerKey = encoded.parse().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn peer_key_roundtrip_through_serde() {
        let key = PeerKey::from_bytes([0x42; 32]);
        let json = serde_json::to_string(&key).unwrap();
        assert!(json.starts_with('"') && json.ends_with('"'));
        let decoded: PeerKey = serde_json::from_str(&json).unwrap();
        assert_eq!(key, decoded);
    }

    #[test]
    fn peer_key_rejects_wrong_length() {
        let too_short = "abcd";
        let result: Result<PeerKey, _> = too_short.parse();
        assert!(matches!(
            result,
            Err(PeerKeyError::InvalidLength {
                expected: 32,
                got: 2
            })
        ));
    }

    #[test]
    fn peer_key_rejects_invalid_hex() {
        let invalid = "not_hex_at_all_not_hex_at_all_not_hex_at_all_not_hex_at_all_xxxx";
        let result: Result<PeerKey, _> = invalid.parse();
        assert!(matches!(result, Err(PeerKeyError::Encoding(_))));
    }

    #[test]
    fn peer_key_as_bytes_returns_original() {
        let bytes = [0x13; 32];
        let key = PeerKey::from_bytes(bytes);
        assert_eq!(key.as_bytes(), &bytes);
    }
}
