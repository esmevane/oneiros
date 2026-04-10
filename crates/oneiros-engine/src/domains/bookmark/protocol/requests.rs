use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateBookmark {
    #[builder(into)]
    pub name: BookmarkName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SwitchBookmark {
    #[builder(into)]
    pub name: BookmarkName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct MergeBookmark {
    /// The source bookmark to merge into the active bookmark.
    #[builder(into)]
    pub source: BookmarkName,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListBookmarks {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

/// Mint a distribution ticket for a bookmark and return a shareable
/// `oneiros://` URI. The ticket is self-contained — anyone holding the URI
/// can present the embedded token to reach the bookmark via this host's
/// iroh endpoint.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ShareBookmark {
    /// The bookmark to share.
    #[builder(into)]
    pub name: BookmarkName,
    /// The actor issuing the share. When omitted, the service picks the
    /// first actor in the host's tenant (matching the project-init
    /// convention). This keeps the common case a single-arg command.
    #[arg(long)]
    pub actor_id: Option<ActorId>,
}

/// Follow a bookmark via a URI. For `ref:` URIs the source is local;
/// for `oneiros://` URIs the source is a peer. The URI's token (for the
/// peer case) is presented during collect to authorize event transfer.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct FollowBookmark {
    /// The URI to follow. Must be either a `ref:` or `oneiros://` form.
    pub uri: String,
    /// Local name for the new bookmark that will mirror the remote.
    #[builder(into)]
    #[arg(long)]
    pub name: BookmarkName,
}

/// Collect events from a followed bookmark's source. For Local follows
/// this reads from the local CanonIndex. For Peer follows this opens an
/// iroh connection and runs the sync protocol.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CollectBookmark {
    #[builder(into)]
    pub name: BookmarkName,
}

/// Remove a follow. The bookmark itself and any previously-collected
/// events stay; only the remote binding is severed.
#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UnfollowBookmark {
    #[builder(into)]
    pub name: BookmarkName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = BookmarkRequestType, display = "kebab-case")]
pub enum BookmarkRequest {
    CreateBookmark(CreateBookmark),
    SwitchBookmark(SwitchBookmark),
    MergeBookmark(MergeBookmark),
    ListBookmarks(ListBookmarks),
    ShareBookmark(ShareBookmark),
    FollowBookmark(FollowBookmark),
    CollectBookmark(CollectBookmark),
    UnfollowBookmark(UnfollowBookmark),
}
