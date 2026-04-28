use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = BookmarkResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BookmarkResponse {
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
    pub enum BookmarkCreatedResponse {
        V1 => { #[serde(flatten)] pub bookmark: Bookmark }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BookmarkForkedResponse {
        V1 => {
            #[serde(flatten)] pub bookmark: Bookmark,
            pub from: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BookmarkSwitchedResponse {
        V1 => {
            pub brain: BrainName,
            pub name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BookmarkMergedResponse {
        V1 => {
            pub brain: BrainName,
            pub source: BookmarkName,
            pub target: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BookmarkUnfollowedResponse {
        V1 => {
            pub follow_id: FollowId,
            pub brain: BrainName,
            pub bookmark: BookmarkName,
        }
    }
}

/// The outcome of a successful `bookmark share` — the minted ticket and
/// the composed URI. The URI is derivable from the ticket plus current
/// host identity, but it's convenient to return it alongside so callers
/// don't need to reconstruct it.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BookmarkShareResult {
    pub ticket: Ticket,
    pub uri: String,
}

/// The outcome of a successful `bookmark collect` — how many events were
/// received and the new checkpoint after applying them.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BookmarkCollectResult {
    pub follow_id: FollowId,
    pub events_received: u64,
    pub checkpoint: Checkpoint,
}
