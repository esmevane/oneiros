use data_encoding::BASE64URL_NOPAD;
use serde::{Deserialize, Serialize};

/// The reachability information for a peer — wraps `iroh::EndpointAddr`.
///
/// Contains the peer's endpoint identifier plus any known relay URLs and
/// direct socket addresses. Used by the Bridge transport layer when opening
/// connections to remote peers.
///
/// This is one of the two files in the engine that imports `iroh::*`. All
/// other code talks to PeerAddress rather than reaching into iroh types
/// directly.
///
/// Displayed as a base64url-encoded postcard blob. Composable into
/// `oneiros://<host>/...` URIs via the host segment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct PeerAddress(iroh::EndpointAddr);

#[derive(Debug, thiserror::Error)]
pub(crate) enum PeerAddressError {
    #[error("invalid peer address encoding: {0}")]
    Encoding(#[from] data_encoding::DecodeError),
    #[error("invalid peer address format: {0}")]
    Format(#[from] postcard::Error),
}

impl PeerAddress {
    pub(crate) fn new(inner: iroh::EndpointAddr) -> Self {
        Self(inner)
    }

    pub(crate) fn inner(&self) -> &iroh::EndpointAddr {
        &self.0
    }

    pub(crate) fn into_inner(self) -> iroh::EndpointAddr {
        self.0
    }

    /// Serialize to postcard bytes. Used when composing URIs.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(&self.0).expect("EndpointAddr serialization should not fail")
    }

    /// Deserialize from postcard bytes. Used when parsing URIs.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, PeerAddressError> {
        let inner = postcard::from_bytes(bytes)?;
        Ok(Self(inner))
    }
}

impl core::fmt::Display for PeerAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", BASE64URL_NOPAD.encode(&self.to_bytes()))
    }
}

impl core::str::FromStr for PeerAddress {
    type Err = PeerAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE64URL_NOPAD.decode(s.as_bytes())?;
        Self::from_bytes(&bytes)
    }
}

impl From<iroh::EndpointAddr> for PeerAddress {
    fn from(addr: iroh::EndpointAddr) -> Self {
        Self(addr)
    }
}

impl schemars::JsonSchema for PeerAddress {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("PeerAddress")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "description": "Base64url-encoded iroh endpoint address (endpoint id + relay + direct addresses)"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_address() -> PeerAddress {
        let secret = iroh::SecretKey::generate(&mut rand::rng());
        let endpoint_id = secret.public();
        PeerAddress::new(iroh::EndpointAddr::new(endpoint_id))
    }

    #[test]
    fn peer_address_roundtrip_through_display() {
        let original = sample_address();
        let encoded = original.to_string();
        let decoded: PeerAddress = encoded.parse().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn peer_address_roundtrip_through_bytes() {
        let original = sample_address();
        let bytes = original.to_bytes();
        let decoded = PeerAddress::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn peer_address_roundtrip_through_serde() {
        let original = sample_address();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: PeerAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn peer_address_rejects_invalid_base64() {
        let result: Result<PeerAddress, _> = "!!!not_base64!!!".parse();
        assert!(result.is_err());
    }
}
