use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = BookmarkResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum BookmarkResponse {
    Created(BookmarkCreated),
    Forked(BookmarkForked),
    Switched(BookmarkSwitched),
    Merged(BookmarkMerged),
    Bookmarks(Listed<Bookmark>),
    Shared(BookmarkShareResult),
    Followed(Follow),
    Collected(BookmarkCollectResult),
    Unfollowed(BookmarkUnfollowed),
}

/// The outcome of a successful `bookmark share` — the minted ticket and
/// the composed URI. The URI is derivable from the ticket plus current
/// host identity, but it's convenient to return it alongside so callers
/// don't need to reconstruct it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkShareResult {
    pub(crate) ticket: Ticket,
    pub(crate) uri: String,
}

/// The outcome of a successful `bookmark collect` — how many events were
/// received and the new checkpoint after applying them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkCollectResult {
    pub(crate) follow_id: FollowId,
    pub(crate) events_received: u64,
    pub(crate) checkpoint: Checkpoint,
}
