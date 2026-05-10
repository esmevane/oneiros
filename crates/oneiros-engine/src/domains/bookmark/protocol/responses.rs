use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = BookmarkResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum BookmarkResponse {
    Created(BookmarkCreatedResponse),
    Forked(BookmarkForkedResponse),
    Switched(BookmarkSwitchedResponse),
    Merged(BookmarkMergedResponse),
    Bookmarks(Listed<Bookmark>),
    Shared(BookmarkShareResult),
    Followed(Follow),
    Collected(BookmarkCollectResult),
    Unfollowed(BookmarkUnfollowedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BookmarkCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) bookmark: Bookmark }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BookmarkForkedResponse {
        V1 => {
            #[serde(flatten)] pub(crate) bookmark: Bookmark,
            pub(crate) from: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BookmarkSwitchedResponse {
        V1 => {
            pub(crate) brain: BrainName,
            pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BookmarkMergedResponse {
        V1 => {
            pub(crate) brain: BrainName,
            pub(crate) source: BookmarkName,
            pub(crate) target: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BookmarkUnfollowedResponse {
        V1 => {
            pub(crate) follow_id: FollowId,
            pub(crate) brain: BrainName,
            pub(crate) bookmark: BookmarkName,
        }
    }
}

/// The outcome of a successful `bookmark share` — the minted ticket and
/// the composed URI. The URI is derivable from the ticket plus current
/// host identity, but it's convenient to return it alongside so callers
/// don't need to reconstruct it.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct BookmarkShareResult {
    pub(crate) ticket: Ticket,
    pub(crate) uri: String,
}

/// The outcome of a successful `bookmark collect` — how many events were
/// received and the new checkpoint after applying them.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct BookmarkCollectResult {
    pub(crate) follow_id: FollowId,
    pub(crate) events_received: u64,
    pub(crate) checkpoint: Checkpoint,
}
