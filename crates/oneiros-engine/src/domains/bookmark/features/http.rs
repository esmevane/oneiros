use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct BookmarkRouter;

impl BookmarkRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/brains/{brain}/bookmarks",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{name}/switch", routing::post(switch))
                .route("/{name}/merge", routing::post(merge))
                .route("/{name}/share", routing::post(share))
                .route("/follow", routing::post(follow))
                .route("/{name}/collect", routing::post(collect))
                .route("/{name}/unfollow", routing::post(unfollow)),
        )
    }
}

async fn create(
    context: SystemContext,
    State(state): State<ServerState>,
    Path(brain): Path<BrainName>,
    Json(body): Json<CreateBookmark>,
) -> Result<(StatusCode, Json<BookmarkResponse>), BookmarkError> {
    let response = BookmarkService::create(&context, state.canons(), &brain, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn switch(
    context: SystemContext,
    State(state): State<ServerState>,
    Path((brain, name)): Path<(BrainName, BookmarkName)>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let request = SwitchBookmark { name };
    let response =
        BookmarkService::switch(&context, state.canons(), state.config(), &brain, &request).await?;
    Ok(Json(response))
}

async fn merge(
    context: SystemContext,
    State(state): State<ServerState>,
    Path((brain, name)): Path<(BrainName, BookmarkName)>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let request = MergeBookmark { source: name };
    let response =
        BookmarkService::merge(&context, state.canons(), state.config(), &brain, &request).await?;
    Ok(Json(response))
}

async fn list(
    context: SystemContext,
    Path(brain): Path<BrainName>,
    Query(params): Query<ListBookmarks>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let response = BookmarkService::list(&context, &brain, &params).await?;
    Ok(Json(response))
}

async fn share(
    context: SystemContext,
    State(state): State<ServerState>,
    Path((brain, name)): Path<(BrainName, BookmarkName)>,
    Json(body): Json<ShareBookmarkBody>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let identity = state.host_identity().ok_or(BookmarkError::NoHostIdentity)?;
    let request = ShareBookmark {
        name,
        actor_id: body.actor_id,
    };
    let response = BookmarkService::share(&context, identity, &brain, &request).await?;
    Ok(Json(response))
}

/// Body of the share POST — the path carries the bookmark name, the
/// body carries the issuing actor id (omit to use the first actor).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct ShareBookmarkBody {
    #[serde(default)]
    pub actor_id: Option<ActorId>,
}

async fn follow(
    context: SystemContext,
    State(state): State<ServerState>,
    Path(brain): Path<BrainName>,
    Json(body): Json<FollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let response = BookmarkService::follow(&context, state.canons(), &brain, &body).await?;
    Ok(Json(response))
}

async fn unfollow(
    context: SystemContext,
    Path((brain, name)): Path<(BrainName, BookmarkName)>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let request = UnfollowBookmark::builder().name(name).build();
    let response = BookmarkService::unfollow(&context, &brain, &request).await?;
    Ok(Json(response))
}

async fn collect(
    context: SystemContext,
    State(state): State<ServerState>,
    Path((brain, name)): Path<(BrainName, BookmarkName)>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let request = CollectBookmark { name };
    let bridge = state.bridge().cloned();
    let canons = state.canons().clone();
    let response =
        BookmarkService::collect(&context, &canons, bridge.as_ref(), &brain, &request).await?;
    Ok(Json(response))
}
