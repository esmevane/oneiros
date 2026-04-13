use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub(crate) struct BookmarkRouter;

impl BookmarkRouter {
    pub(crate) fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/bookmarks",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/switch", routing::post(switch))
                .route("/merge", routing::post(merge))
                .route("/share", routing::post(share))
                .route("/follow", routing::post(follow))
                .route("/collect", routing::post(collect))
                .route("/unfollow", routing::post(unfollow)),
        )
    }
}

async fn create(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<CreateBookmark>,
) -> Result<(StatusCode, Json<BookmarkResponse>), BookmarkError> {
    let response = BookmarkService::create(&state, context.brain_name(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn switch(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<SwitchBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::switch(&state, context.brain_name(), &body).await?,
    ))
}

async fn merge(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<MergeBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::merge(&state, context.brain_name(), &body).await?,
    ))
}

async fn list(
    context: ProjectContext,
    State(state): State<ServerState>,
    Query(params): Query<ListBookmarks>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::list(&state, context.brain_name(), &params).await?,
    ))
}

async fn share(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<ShareBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::share(&state, context.brain_name(), &body).await?,
    ))
}

async fn follow(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<FollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::follow(&state, context.brain_name(), &body).await?,
    ))
}

async fn unfollow(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<UnfollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::unfollow(&state, context.brain_name(), &body).await?,
    ))
}

async fn collect(
    context: ProjectContext,
    State(state): State<ServerState>,
    Json(body): Json<CollectBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::collect(&state, context.brain_name(), &body).await?,
    ))
}
