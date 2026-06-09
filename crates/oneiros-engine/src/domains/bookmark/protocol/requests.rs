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

impl ClientRequest for CreateBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks", self).await
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

impl ClientRequest for SwitchBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/switch", self).await
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

impl ClientRequest for MergeBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/merge", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListBookmarks {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListBookmarks {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListBookmarks::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/bookmarks?{query}")).await
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

impl ClientRequest for ShareBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/share", self).await
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

impl ClientRequest for FollowBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/follow", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CollectBookmark {
        #[derive(clap::Args)]
        V1 => {
            /// For follow-based collect: the local bookmark name.
            /// For remote collect: the bookmark name on the remote.
            #[builder(into)] pub(crate) name: BookmarkName,
            /// Collect from a remote host instead of a follow source.
            #[arg(long)]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) remote: Option<RemoteName>,
            /// Local name to assign when collecting from a remote.
            /// Defaults to the name field.
            #[arg(long = "as")]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) as_name: Option<BookmarkName>,
        }
    }
}

impl ClientRequest for CollectBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/collect", self).await
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

impl ClientRequest for UnfollowBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/unfollow", self).await
    }
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
    PushBookmark(PushBookmark),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PushBookmark {
        #[derive(clap::Args)]
        V1 => {
            /// Name of the remote to push to.
            #[builder(into)] pub(crate) remote: RemoteName,
            /// Local bookmark name to push.
            #[builder(into)] pub(crate) name: BookmarkName,
            /// Rename the bookmark on the remote.
            #[arg(long = "as")]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub(crate) as_name: Option<BookmarkName>,
        }
    }
}

impl ClientRequest for PushBookmark {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/bookmarks/push", self).await
    }
}
