use data_encoding::BASE64URL_NOPAD;
use serde::{Deserialize, Serialize};

use crate::*;

/// A resource through a ticket. Pairs a target reference (what you're pointing
/// at) with an authorization token (how you're allowed to reach it).
///
/// Links are the "resource + authorization" primitive used by distribution URIs.
/// They're self-contained: a holder has everything needed to present the token
/// at a host and receive the resource in question.
///
/// Displayed as `link:<base64url>` where the payload is the postcard-encoded
/// `Link` struct. Matches the `ref:<base64url>` convention of `RefToken`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub(crate) struct Link {
    pub(crate) target: Ref,
    pub(crate) token: Token,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LinkError {
    #[error("invalid link encoding: {0}")]
    Encoding(#[from] data_encoding::DecodeError),
    #[error("invalid link format: {0}")]
    Format(#[from] postcard::Error),
}

impl Link {
    pub(crate) fn new(target: Ref, token: Token) -> Self {
        Self { target, token }
    }

    /// Encode this link to postcard bytes.
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        postcard::to_allocvec(self).expect("link serialization should not fail")
    }

    /// Decode a link from postcard bytes.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, LinkError> {
        Ok(postcard::from_bytes(bytes)?)
    }
}

impl core::fmt::Display for Link {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "link:{}", BASE64URL_NOPAD.encode(&self.to_bytes()))
    }
}

impl core::str::FromStr for Link {
    type Err = LinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let encoded = s.strip_prefix("link:").unwrap_or(s);
        let bytes = BASE64URL_NOPAD.decode(encoded.as_bytes())?;
        Self::from_bytes(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_token() -> Token {
        Token::from("testtokenvalue")
    }

    #[test]
    fn link_display_has_link_prefix() {
        let link = Link::new(Ref::bookmark(BookmarkId::new()), sample_token());
        let display = link.to_string();
        assert!(display.starts_with("link:"), "got: {display}");
    }

    #[test]
    fn link_roundtrip_through_display() {
        let original = Link::new(Ref::bookmark(BookmarkId::new()), sample_token());
        let encoded = original.to_string();
        let decoded: Link = encoded.parse().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn link_accepts_without_prefix() {
        let original = Link::new(Ref::bookmark(BookmarkId::new()), sample_token());
        let full = original.to_string();
        let raw = full.strip_prefix("link:").unwrap();
        let decoded: Link = raw.parse().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn link_roundtrip_through_bytes() {
        let original = Link::new(
            Ref::cognition(CognitionId::new()),
            Token::from("anothertoken"),
        );
        let bytes = original.to_bytes();
        let decoded = Link::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn link_roundtrip_through_serde() {
        let original = Link::new(Ref::peer(PeerId::new()), sample_token());
        let json = serde_json::to_string(&original).unwrap();
        let decoded: Link = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn link_rejects_invalid_base64() {
        let result: Result<Link, _> = "link:!!!not_valid!!!".parse();
        assert!(result.is_err());
    }

    #[test]
    fn link_rejects_invalid_postcard() {
        let result: Result<Link, _> = "link:AAAA".parse();
        assert!(matches!(result, Err(LinkError::Format(_))));
    }
}
