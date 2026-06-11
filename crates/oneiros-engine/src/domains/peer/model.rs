use bon::Builder;
use core::str::FromStr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// What kind of relationship this peer represents.
///
/// Infered automatically from the ticket URI when the peer is added.
///
/// - `Bookmark` — a bookmark-scoped peer, created by `bookmark follow`.
///   The ticket targets a specific bookmark; the peer can be used for
///   collection but not submission.
/// - `Repository` — a project-scoped peer, created by `peer add` with a
///   project ticket URI or `peer share`. The ticket targets a project;
///   the peer supports submission and bookmark listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum PeerKind {
    #[default]
    Bookmark,
    Project,
}

impl core::fmt::Display for PeerKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bookmark => f.write_str("bookmark"),
            Self::Project => f.write_str("project"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unable to determine peer kind from '#{0}'")]
pub(crate) struct PeerKindParseFailure(String);

impl FromStr for PeerKind {
    type Err = PeerKindParseFailure;

    fn from_str(given_string: &str) -> Result<Self, Self::Err> {
        Ok(match given_string {
            "project" => PeerKind::Project,
            "bookmark" => PeerKind::Bookmark,
            _ => return Err(PeerKindParseFailure(given_string.to_string())),
        })
    }
}

/// A known remote host — the persisted identity of a peer this host has
/// learned about.
///
/// `id` is the internal domain handle (UUID), used for references and
/// local bookkeeping. `key` is the cryptographic identity (ed25519 public
/// key) — unforgeable, the thing iroh actually verifies during connection
/// establishment. `address` is the current reachability info (may change
/// as the peer's network environment shifts). `name` is the human-readable
/// label for display, defaulting to a short hex prefix of the key when no
/// explicit name is provided.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Peer {
    #[builder(default)]
    pub(crate) id: PeerId,
    pub(crate) key: PeerKey,
    pub(crate) address: PeerAddress,
    #[builder(into)]
    pub(crate) name: PeerName,
    #[builder(default)]
    pub(crate) kind: PeerKind,
    /// Project-scoped ticket (only for Remote peers).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) ticket: Option<Link>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) project: Option<ProjectName>,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Indexable<PeerId> for Peer {
    fn id(&self) -> PeerId {
        self.id
    }
}

pub(crate) type Peers = EntityIndex<PeerId, Peer>;

resource_id!(PeerId);
resource_name!(PeerName);
