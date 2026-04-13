use crate::*;

/// A resource on a peer host, reached through that peer's transport and
/// authorized by a ticket. Bundles the peer's reachability (`PeerAddress`)
/// with a `Link` (target + token).
///
/// This is the external form of a distribution reference — what you get from
/// `bookmark share` and what you hand to `bookmark follow`. Displays as an
/// `oneiros://<host>/link:<payload>` URI.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct PeerLink {
    pub(crate) host: PeerAddress,
    pub(crate) link: Link,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PeerLinkError {
    #[error("missing oneiros:// scheme prefix")]
    MissingScheme,
    #[error("missing path separator after host segment")]
    MissingPath,
    #[error("invalid host segment: {0}")]
    Host(#[from] PeerAddressError),
    #[error("invalid link segment: {0}")]
    Link(#[from] LinkError),
}

impl PeerLink {
    pub(crate) fn new(host: PeerAddress, link: Link) -> Self {
        Self { host, link }
    }
}

impl core::fmt::Display for PeerLink {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "oneiros://{}/{}", self.host, self.link)
    }
}

impl core::str::FromStr for PeerLink {
    type Err = PeerLinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s
            .strip_prefix("oneiros://")
            .ok_or(PeerLinkError::MissingScheme)?;
        let (host_part, link_part) = rest.split_once('/').ok_or(PeerLinkError::MissingPath)?;
        let host = host_part.parse::<PeerAddress>()?;
        let link = link_part.parse::<Link>()?;
        Ok(Self { host, link })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_peer_link() -> PeerLink {
        let secret = iroh::SecretKey::generate(&mut rand::rng());
        let endpoint_id = secret.public();
        let host = PeerAddress::new(iroh::EndpointAddr::new(endpoint_id));
        let link = Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken"));
        PeerLink::new(host, link)
    }

    #[test]
    fn peer_link_display_has_oneiros_scheme() {
        let pl = sample_peer_link();
        let display = pl.to_string();
        assert!(display.starts_with("oneiros://"), "got: {display}");
    }

    #[test]
    fn peer_link_display_contains_link_prefix() {
        let pl = sample_peer_link();
        let display = pl.to_string();
        assert!(display.contains("/link:"), "got: {display}");
    }

    #[test]
    fn peer_link_roundtrip_through_display() {
        let original = sample_peer_link();
        let encoded = original.to_string();
        let decoded: PeerLink = encoded.parse().unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn peer_link_rejects_missing_scheme() {
        let result: Result<PeerLink, _> = "ref:abc".parse();
        assert!(matches!(result, Err(PeerLinkError::MissingScheme)));
    }

    #[test]
    fn peer_link_rejects_missing_path() {
        let result: Result<PeerLink, _> = "oneiros://nopath".parse();
        assert!(matches!(result, Err(PeerLinkError::MissingPath)));
    }

    #[test]
    fn peer_link_roundtrip_through_serde() {
        let original = sample_peer_link();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: PeerLink = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}
