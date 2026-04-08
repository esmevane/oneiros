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
                .route("/{name}/merge", routing::post(merge)),
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
