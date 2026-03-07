use std::{fmt, net::SocketAddr};

use serde::{Deserialize, Serialize};

/// A `TrustMode` with `Auto` resolved to a concrete choice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ResolvedMode {
    /// Local self-signed CA.
    Local,
    /// ACME / Let's Encrypt.
    Acme,
    /// TLS disabled.
    Off,
}

/// A network endpoint address, e.g. `"127.0.0.1:2100"`.
///
/// # Examples
///
/// ```
/// use oneiros_trust::Endpoint;
/// use std::net::SocketAddr;
///
/// let addr: SocketAddr = "127.0.0.1:2100".parse().unwrap();
/// let endpoint = Endpoint::from(addr);
/// assert_eq!(endpoint.to_string(), "127.0.0.1:2100");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint(pub String);

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<SocketAddr> for Endpoint {
    fn from(addr: SocketAddr) -> Self {
        Self(addr.to_string())
    }
}

impl From<String> for Endpoint {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Endpoint {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

/// A hex-encoded certificate fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fingerprint(pub String);

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Fingerprint {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// The trust status of a peer connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustStatus {
    /// Connection is TLS-secured and the peer is trusted.
    Secure,
    /// Connection is not TLS-secured but the endpoint is explicitly allowed.
    InsecureAllowed,
    /// Trust state is not yet determined.
    Unknown,
    /// The peer's certificate fingerprint does not match the expected value.
    Untrusted {
        expected: Fingerprint,
        actual: Fingerprint,
    },
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_mode_serde_roundtrip() {
        use oneiros_config::TrustMode;
        let serialized = serde_json::to_string(&TrustMode::Acme).unwrap();
        assert_eq!(serialized, "\"acme\"");

        let deserialized: TrustMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, TrustMode::Acme);
    }

    #[test]
    fn trust_mode_default_is_off() {
        use oneiros_config::TrustMode;
        assert_eq!(TrustMode::default(), TrustMode::Off);
    }

    #[test]
    fn endpoint_from_socket_addr() {
        let addr: SocketAddr = "127.0.0.1:2100".parse().unwrap();
        let endpoint = Endpoint::from(addr);
        assert_eq!(endpoint.to_string(), "127.0.0.1:2100");
    }
}
