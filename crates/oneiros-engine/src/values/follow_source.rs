use serde::{Deserialize, Serialize};

use crate::*;

/// Where a follow fetches events from. Two modalities, typed into the
/// enum so `BookmarkService::collect` can dispatch without runtime
/// classification.
///
/// - **`Local(Ref)`** — the source is resolvable within the current host's
///   `CanonIndex`. No transport, no ticket. Used for same-host follows
///   (whether same-brain or different-brain on this system).
/// - **`Peer(PeerLink)`** — the source is on another host, reached via
///   iroh transport through the Bridge. The PeerLink carries both the host
///   address and the authorization token.
///
/// A Link-only variant (host-internal with a ticket but no transport) is
/// intentionally omitted — we don't currently have a use case for it.
/// The shape parallels [`OneirosUri`] if it's ever needed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum FollowSource {
    Local(Ref),
    Peer(PeerLink),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn sample_peer_link() -> PeerLink {
        let secret = iroh::SecretKey::generate();
        let endpoint_id = secret.public();
        let host = PeerAddress::new(iroh::EndpointAddr::new(endpoint_id));
        let link = Link::new(Ref::bookmark(BookmarkId::new()), Token::from("testtoken"));
        PeerLink::new(host, link)
    }

    #[test]
    fn local_source_roundtrip_through_serde() {
        let original = FollowSource::Local(Ref::bookmark(BookmarkId::new()));
        let json = serde_json::to_string(&original).unwrap();
        let decoded: FollowSource = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn peer_source_roundtrip_through_serde() {
        let original = FollowSource::Peer(sample_peer_link());
        let json = serde_json::to_string(&original).unwrap();
        let decoded: FollowSource = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}
