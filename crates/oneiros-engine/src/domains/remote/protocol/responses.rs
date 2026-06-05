use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = RemoteResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum RemoteResponse {
    Added(RemoteAddedResponse),
    Found(RemoteFoundResponse),
    Listed(RemotesResponse),
    Removed(RemoteRemovedResponse),
    Bookmarks(RemoteBookmarkListResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoteAddedResponse {
        V1 => { #[serde(flatten)] pub(crate) remote: Remote }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoteFoundResponse {
        V1 => { #[serde(flatten)] pub(crate) remote: Remote }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemotesResponse {
        V1 => {
            pub(crate) items: Vec<Remote>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoteRemovedResponse {
        V1 => {
            pub(crate) id: RemoteId,
            pub(crate) name: RemoteName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoteBookmarkListResponse {
        V1 => {
            pub(crate) bookmarks: Vec<BookmarkName>,
        }
    }
}
