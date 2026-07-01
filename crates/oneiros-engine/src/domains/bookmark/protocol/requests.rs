use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use aide::axum::routing::{self, ApiMethodRouter};
use aide::transform::TransformOperation;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

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

impl CreateBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<CreateBookmark>,
    ) -> Result<(StatusCode, Json<BookmarkResponse>), BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        let response =
            BookmarkService::create(&scope, &state, context.project_name(), &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

resource_meta! {
    CreateBookmark => {
        path: "/",
        summary: "Create a bookmark",
        description: "Create a new bookmark that defines a named view of the event timeline.",
        content: include_str!("../features/skills/create.md"),
        status: 201,
    }
}

resource_handler! {
    CreateBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<201, Json<BookmarkCreatedResponse>>(),
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

impl SwitchBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<SwitchBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::switch(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    SwitchBookmark => {
        path: "/switch",
        summary: "Switch to a bookmark",
        description: "Set the active bookmark, making its timeline view the current working context.",
        content: include_str!("../features/skills/switch.md"),
        status: 200,
    }
}

resource_handler! {
    SwitchBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<BookmarkSwitchedResponse>>(),
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

impl MergeBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<MergeBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::merge(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    MergeBookmark => {
        path: "/merge",
        summary: "Merge a bookmark",
        description: "Integrate the events from a bookmark into the current active timeline.",
        content: include_str!("../features/skills/merge.md"),
        status: 200,
    }
}

resource_handler! {
    MergeBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<BookmarkMergedResponse>>(),
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

impl ListBookmarks {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Query(params): Query<ListBookmarks>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::list(&scope, &state, context.project_name(), &params).await?,
        ))
    }
}

resource_meta! {
    ListBookmarks => {
        path: "/",
        summary: "List bookmarks",
        description: "List all bookmarks known to the current project.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    }
}

resource_handler! {
    ListBookmarks => {
        handler: Self::handler,
        method: ResourceMethod::Get,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<Listed<Bookmark>>>(),
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

impl ShareBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<ShareBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::share(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    ShareBookmark => {
        path: "/share",
        summary: "Share a bookmark",
        description: "Produce a shareable oneiros:// link representing this bookmark",
        content: include_str!("../features/skills/share.md"),
        status: 200,
    }
}

resource_handler! {
    ShareBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<BookmarkShareResult>>(),
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

impl FollowBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<FollowBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::follow(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    FollowBookmark => {
        path: "/follow",
        summary: "Follow a bookmark link",
        description: "Create a local bookmark by following a remote oneiros:// link.",
        content: include_str!("../features/skills/follow.md"),
        status: 200,
    }
}

resource_handler! {
    FollowBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<Follow>>(),
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

impl CollectBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<CollectBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::collect(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    CollectBookmark => {
        path: "/collect",
        summary: "Collect events into a bookmark",
        description: "Collect events from a followed source or directly from a peer host.",
        content: include_str!("../features/skills/collect.md"),
        status: 200,
    }
}

resource_handler! {
    CollectBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<BookmarkCollectResult>>(),
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

impl UnfollowBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<UnfollowBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::unfollow(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    UnfollowBookmark => {
        path: "/unfollow",
        summary: "Unfollow a bookmark",
        description: "Remove a followed bookmark, stopping incremental collection.",
        content: include_str!("../features/skills/unfollow.md"),
        status: 200,
    }
}

resource_handler! {
    UnfollowBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<BookmarkUnfollowedResponse>>(),
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

impl SubmitBookmark {
    pub(crate) async fn handler(
        context: ProjectLog,
        State(state): State<ServerState>,
        Json(body): Json<SubmitBookmark>,
    ) -> Result<Json<BookmarkResponse>, BookmarkError> {
        let scope = ComposeScope::new(state.config().clone()).host()?;
        Ok(Json(
            BookmarkService::submit(&scope, &state, context.project_name(), &body).await?,
        ))
    }
}

resource_meta! {
    SubmitBookmark => {
        path: "/submit",
        summary: "Submit a bookmark to a remote",
        description: "Submit a bookmark to a peer host.",
        content: include_str!("../features/skills/submit.md"),
        status: 200,
    }
}

resource_handler! {
    SubmitBookmark => {
        handler: Self::handler,
        method: ResourceMethod::Post,
        transform: |op| op.security_requirement("BearerToken").response::<200, Json<BookmarkSubmitResult>>(),
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

resource_root! {
    BookmarkRequest => {
        meta: { label: "bookmarks", summary: "Manage timeline bookmarks" },
        operations: {
            match given_kind => {
                BookmarkRequestType::CreateBookmark => CreateBookmark,
                BookmarkRequestType::SwitchBookmark => SwitchBookmark,
                BookmarkRequestType::MergeBookmark => MergeBookmark,
                BookmarkRequestType::ListBookmarks => ListBookmarks,
                BookmarkRequestType::ShareBookmark => ShareBookmark,
                BookmarkRequestType::FollowBookmark => FollowBookmark,
                BookmarkRequestType::CollectBookmark => CollectBookmark,
                BookmarkRequestType::UnfollowBookmark => UnfollowBookmark,
                BookmarkRequestType::SubmitBookmark => SubmitBookmark,
            }
        }
    }
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
