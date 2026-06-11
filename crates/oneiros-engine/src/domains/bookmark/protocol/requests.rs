use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
            #[builder(default)]
            pub(crate) event_ids: Vec<EventId>,
            #[arg(long = "from-slice")]
            #[builder(into)]
            pub(crate) from_slice: Option<SliceName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SwitchBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum MergeBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) source: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListBookmarks {
        #[derive(clap::Args)]
        V2 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
            /// List bookmarks from a peer instead of locally.
            #[arg(long, alias = "peer")]
            pub(crate) from: Option<PeerName>,
        },
        #[derive(clap::Args, schemars::JsonSchema)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        },
    }
}

impl TryFrom<ListBookmarksV1> for ListBookmarksV2 {
    type Error = UpcastError;
    fn try_from(v1: ListBookmarksV1) -> Result<Self, Self::Error> {
        Ok(ListBookmarksV2 {
            filters: v1.filters,
            from: None,
        })
    }
}

impl From<ListBookmarksV1> for ListBookmarks {
    fn from(v: ListBookmarksV1) -> Self {
        Self::V1(v)
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ShareBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
            #[arg(long)]
            pub(crate) actor_id: Option<ActorId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum FollowBookmark {
        #[derive(clap::Args)]
        V1 => {
            pub(crate) uri: String,
            #[arg(long)]
            #[builder(into)]
            pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CollectBookmark {
        #[derive(clap::Args)]
        V2 => {
            /// For follow-based collect: the local bookmark name.
            /// For peer collect: the bookmark name on the remote.
            #[builder(into)] pub(crate) name: BookmarkName,
            /// Collect from a peer instead of a follow source.
            #[arg(long, alias = "peer")]
            pub(crate) from: Option<PeerName>,
            /// Local name to assign when collecting from a peer.
            /// Defaults to the peer bookmark name.
            #[arg(long = "as")]
            pub(crate) as_name: Option<BookmarkName>,
        },
        #[derive(clap::Args, schemars::JsonSchema)]
        V1 => {
            /// For follow-based collect: the local bookmark name.
            #[builder(into)] pub(crate) name: BookmarkName,
        },
    }
}

impl TryFrom<CollectBookmarkV1> for CollectBookmarkV2 {
    type Error = UpcastError;
    fn try_from(v1: CollectBookmarkV1) -> Result<Self, Self::Error> {
        Ok(CollectBookmarkV2 {
            name: v1.name,
            from: None,
            as_name: None,
        })
    }
}

impl From<CollectBookmarkV1> for CollectBookmark {
    fn from(v: CollectBookmarkV1) -> Self {
        Self::V1(v)
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UnfollowBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SubmitBookmark {
        #[derive(clap::Args)]
        V2 => {
            /// Name of the peer to submit to.
            #[builder(into)] pub(crate) peer: PeerName,
            /// Local bookmark name to submit.
            #[builder(into)] pub(crate) name: BookmarkName,
            /// Rename the bookmark on the remote.
            #[arg(long = "as")]
            #[serde(default)]
            pub(crate) as_name: Option<BookmarkName>,
        },
        #[derive(clap::Args, schemars::JsonSchema)]
        V1 => {
            /// Name of the peer to submit to.
            #[builder(into)] pub(crate) peer: PeerName,
            /// Local bookmark name to submit.
            #[builder(into)] pub(crate) name: BookmarkName,
        },
    }
}

resource_requests! {
    CreateBookmark => |this, client| { client.post("/bookmarks", this).await },
    SwitchBookmark => |this, client| { client.post("/bookmarks/switch", this).await },
    MergeBookmark => |this, client| { client.post("/bookmarks/merge", this).await },
    ListBookmarks => |this, client| {
        match this {
            ListBookmarks::V2(v2) => {
                let mut query = format!("limit={}&offset={}", v2.filters.limit, v2.filters.offset,);
                if let Some(ref from) = v2.from {
                    query.push_str(&format!("&from={}", from));
                }
                client.get(&format!("/bookmarks?{query}")).await
            }
            ListBookmarks::V1(v1) => {
                let query = format!("limit={}&offset={}", v1.filters.limit, v1.filters.offset,);
                client.get(&format!("/bookmarks?{query}")).await
            }
        }
    },
    ShareBookmark => |this, client| { client.post("/bookmarks/share", this).await },
    FollowBookmark => |this, client| { client.post("/bookmarks/follow", this).await },
    CollectBookmark => |this, client| { client.post("/bookmarks/collect", this).await },
    UnfollowBookmark => |this, client| { client.post("/bookmarks/unfollow", this).await },
    SubmitBookmark => |this, client| { client.post("/bookmarks/submit", this).await }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = BookmarkRequestType, display = "kebab-case")]
pub(crate) enum BookmarkRequest {
    CreateBookmark(CreateBookmark),
    SwitchBookmark(SwitchBookmark),
    MergeBookmark(MergeBookmark),
    ListBookmarks(ListBookmarks),
    ShareBookmark(ShareBookmark),
    FollowBookmark(FollowBookmark),
    CollectBookmark(CollectBookmark),
    UnfollowBookmark(UnfollowBookmark),
    SubmitBookmark(SubmitBookmark),
}

impl TryFrom<SubmitBookmarkV1> for SubmitBookmarkV2 {
    type Error = UpcastError;
    fn try_from(v1: SubmitBookmarkV1) -> Result<Self, Self::Error> {
        Ok(SubmitBookmarkV2 {
            peer: v1.peer,
            name: v1.name,
            as_name: None,
        })
    }
}

impl From<SubmitBookmarkV1> for SubmitBookmark {
    fn from(v: SubmitBookmarkV1) -> Self {
        Self::V1(v)
    }
}
