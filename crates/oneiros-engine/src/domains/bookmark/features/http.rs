use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::*;

pub struct BookmarkRouter;

impl BookmarkRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/bookmarks",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, BookmarkDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(create, |op| {
                        resource_op!(op, BookmarkDocs::Create)
                            .security_requirement("BearerToken")
                            .response::<201, Json<BookmarkResponse>>()
                    }),
                )
                .api_route(
                    "/switch",
                    routing::post_with(switch, |op| {
                        resource_op!(op, BookmarkDocs::Switch).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/merge",
                    routing::post_with(merge, |op| {
                        resource_op!(op, BookmarkDocs::Merge).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/share",
                    routing::post_with(share, |op| {
                        resource_op!(op, BookmarkDocs::Share).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/follow",
                    routing::post_with(follow, |op| {
                        resource_op!(op, BookmarkDocs::Follow).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/collect",
                    routing::post_with(collect, |op| {
                        resource_op!(op, BookmarkDocs::Collect).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/unfollow",
                    routing::post_with(unfollow, |op| {
                        resource_op!(op, BookmarkDocs::Unfollow).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn create(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<CreateBookmark>,
) -> Result<(StatusCode, Json<BookmarkResponse>), BookmarkError> {
    let response = BookmarkService::create(&state, context.brain_name(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn switch(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<SwitchBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::switch(&state, context.brain_name(), &body).await?,
    ))
}

async fn merge(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<MergeBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::merge(&state, context.brain_name(), &body).await?,
    ))
}

async fn list(
    context: ProjectLog,
    State(state): State<ServerState>,
    Query(params): Query<ListBookmarks>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::list(&state, context.brain_name(), &params).await?,
    ))
}

async fn share(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<ShareBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::share(&state, context.brain_name(), &body).await?,
    ))
}

async fn follow(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<FollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::follow(&state, context.brain_name(), &body).await?,
    ))
}

async fn unfollow(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<UnfollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::unfollow(&state, context.brain_name(), &body).await?,
    ))
}

async fn collect(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<CollectBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::collect(&state, context.brain_name(), &body).await?,
    ))
}
