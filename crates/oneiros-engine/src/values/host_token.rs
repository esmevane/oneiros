//! A host-scoped bearer token for authenticating local clients (dashboard,
//! CLI) to the HTTP API without requiring a brain-specific ticket.
//!
//! Generated via `HMAC-SHA256(host_secret_key, b"oneiros:host-token:v1")`,
//! base32-encoded. Deterministic from the host key — stable across restarts,
//! changes only if the host key is regenerated. Stateless verification (no
//! DB lookup needed).

/// Context string for domain separation — changing this produces
/// incompatible tokens, allowing versioned rotation.
const HOST_TOKEN_CONTEXT: &[u8] = b"oneiros:host-token:v1";

/// A host-level bearer token. Wraps a base32-encoded HMAC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HostToken(String);

impl HostToken {
    /// The expected base32-encoded length for the HMAC-SHA256 output
    /// (32 bytes → 52 base32 characters, no padding).
    #[allow(dead_code)]
    const ENCODED_LENGTH: usize = 52;

    /// Generate a host token from the host's ed25519 secret key.
    pub(crate) fn generate(secret: &iroh::SecretKey) -> Self {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let mut mac = Hmac::<Sha256>::new_from_slice(&secret.to_bytes())
            .expect("HMAC should accept any key length; 32 bytes is sufficient");
        mac.update(HOST_TOKEN_CONTEXT);
        let tag = mac.finalize().into_bytes();
        Self(data_encoding::BASE32_NOPAD.encode(&tag).to_lowercase())
    }

    /// Verify this token against the host secret key. Returns `true` if
    /// the token is a valid HMAC for the fixed context string.
    pub(crate) fn verify(&self, secret: &iroh::SecretKey) -> bool {
        Self::generate(secret) == *self
    }
}

impl core::fmt::Display for HostToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for HostToken {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_same_key_produces_same_token() {
        let secret = iroh::SecretKey::generate();
        let token1 = HostToken::generate(&secret);
        let token2 = HostToken::generate(&secret);
        assert_eq!(token1, token2, "same key should produce identical tokens");
    }

    #[test]
    fn different_keys_produce_different_tokens() {
        let secret1 = iroh::SecretKey::generate();
        let secret2 = iroh::SecretKey::generate();
        let token1 = HostToken::generate(&secret1);
        let token2 = HostToken::generate(&secret2);
        assert_ne!(
            token1, token2,
            "different keys should produce different tokens"
        );
    }

    #[test]
    fn verification_succeeds_with_correct_secret() {
        let secret = iroh::SecretKey::generate();
        let token = HostToken::generate(&secret);
        assert!(
            token.verify(&secret),
            "should verify against the generating key"
        );
    }

    #[test]
    fn verification_fails_with_wrong_secret() {
        let secret1 = iroh::SecretKey::generate();
        let secret2 = iroh::SecretKey::generate();
        let token = HostToken::generate(&secret1);
        assert!(!token.verify(&secret2), "should reject a different key");
    }

    #[test]
    fn encoded_length_is_predictable() {
        let secret = iroh::SecretKey::generate();
        let token = HostToken::generate(&secret);
        assert_eq!(
            token.to_string().len(),
            HostToken::ENCODED_LENGTH,
            "HMAC-SHA256 output is 32 bytes → 52 base32 chars (no padding)"
        );
    }

    #[test]
    fn roundtrip_through_string() {
        let secret = iroh::SecretKey::generate();
        let token = HostToken::generate(&secret);
        let as_string: String = token.to_string();
        let rehydrated = HostToken::from(as_string);
        assert_eq!(token, rehydrated);
        assert!(rehydrated.verify(&secret));
    }
}
