use crate::*;

/// The unified URI type for oneiros content references.
///
/// Three tiers, distinguished by prefix:
///
/// - **`ref:<base64url>`** — a project-internal reference. Resolves within
///   the current brain; no authorization needed. See [`Ref`] and [`RefToken`].
/// - **`link:<base64url>`** — a host-internal link. Pairs a target [`Ref`]
///   with an authorization [`Token`]. Resolves on the current host, crosses
///   brain boundaries through a ticket. See [`Link`].
/// - **`oneiros://<host>/link:<base64url>`** — an external link. Adds a
///   [`PeerAddress`] host segment for transport routing through iroh. See
///   [`PeerLink`].
///
/// Parsing dispatches on the prefix. Display delegates to the variant's
/// own [`core::fmt::Display`] implementation.
#[derive(Clone, Debug, PartialEq)]
pub enum OneirosUri {
    Ref(Ref),
    Link(Link),
    Peer(PeerLink),
}

#[derive(Debug, thiserror::Error)]
pub enum OneirosUriError {
    #[error("unrecognized uri scheme (expected one of: ref:, link:, oneiros://)")]
    UnknownScheme,
    #[error("invalid ref: {0}")]
    Ref(#[from] RefError),
    #[error("invalid link: {0}")]
    Link(#[from] LinkError),
    #[error("invalid peer link: {0}")]
    Peer(#[from] PeerLinkError),
}

impl OneirosUri {
    /// Returns true if this URI references a resource on another host.
    pub fn is_external(&self) -> bool {
        matches!(self, Self::Peer(_))
    }
}

impl core::fmt::Display for OneirosUri {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ref(r) => write!(f, "{}", RefToken::new(r.clone())),
            Self::Link(l) => write!(f, "{l}"),
            Self::Peer(p) => write!(f, "{p}"),
        }
    }
}

impl core::str::FromStr for OneirosUri {
    type Err = OneirosUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("oneiros://") {
            Ok(Self::Peer(s.parse::<PeerLink>()?))
        } else if s.starts_with("link:") {
            Ok(Self::Link(s.parse::<Link>()?))
        } else if s.starts_with("ref:") {
            let token: RefToken = s.parse()?;
            Ok(Self::Ref(token.into_inner()))
        } else {
            Err(OneirosUriError::UnknownScheme)
        }
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
    fn ref_variant_roundtrip() {
        let r = Ref::cognition(CognitionId::new());
        let uri = OneirosUri::Ref(r.clone());
        let encoded = uri.to_string();
        assert!(encoded.starts_with("ref:"));
        let decoded: OneirosUri = encoded.parse().unwrap();
        assert_eq!(uri, decoded);
        assert!(!decoded.is_external());
    }

    #[test]
    fn link_variant_roundtrip() {
        let link = Link::new(
            Ref::bookmark(BookmarkId::new()),
            Token::from("somelinktoken"),
        );
        let uri = OneirosUri::Link(link);
        let encoded = uri.to_string();
        assert!(encoded.starts_with("link:"));
        let decoded: OneirosUri = encoded.parse().unwrap();
        assert_eq!(uri, decoded);
        assert!(!decoded.is_external());
    }

    #[test]
    fn peer_variant_roundtrip() {
        let pl = sample_peer_link();
        let uri = OneirosUri::Peer(pl);
        let encoded = uri.to_string();
        assert!(encoded.starts_with("oneiros://"));
        assert!(encoded.contains("/link:"));
        let decoded: OneirosUri = encoded.parse().unwrap();
        assert_eq!(uri, decoded);
        assert!(decoded.is_external());
    }

    #[test]
    fn unknown_scheme_errors() {
        let result: Result<OneirosUri, _> = "http://example.com".parse();
        assert!(matches!(result, Err(OneirosUriError::UnknownScheme)));
    }

    #[test]
    fn empty_string_errors() {
        let result: Result<OneirosUri, _> = "".parse();
        assert!(matches!(result, Err(OneirosUriError::UnknownScheme)));
    }

    #[test]
    fn dispatches_on_prefix_ref() {
        let r = Ref::texture(TextureName::new("observation"));
        let encoded = RefToken::new(r.clone()).to_string();
        let uri: OneirosUri = encoded.parse().unwrap();
        assert!(matches!(uri, OneirosUri::Ref(_)));
    }
}
