use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::*;

pub(crate) struct BookmarkRouter;

impl BookmarkRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/bookmarks",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, BookmarkDocs::List)
                            .security_requirement("BearerToken")
                            .response::<200, Json<Listed<Bookmark>>>()
                    })
                    .post_with(create, |op| {
                        resource_op!(op, BookmarkDocs::Create)
                            .security_requirement("BearerToken")
                            .response::<201, Json<BookmarkCreatedResponse>>()
                    }),
                )
                .api_route(
                    "/switch",
                    routing::post_with(switch, |op| {
                        resource_op!(op, BookmarkDocs::Switch)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkSwitchedResponse>>()
                    }),
                )
                .api_route(
                    "/merge",
                    routing::post_with(merge, |op| {
                        resource_op!(op, BookmarkDocs::Merge)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkMergedResponse>>()
                    }),
                )
                .api_route(
                    "/share",
                    routing::post_with(share, |op| {
                        resource_op!(op, BookmarkDocs::Share)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkShareResult>>()
                    }),
                )
                .api_route(
                    "/follow",
                    routing::post_with(follow, |op| {
                        resource_op!(op, BookmarkDocs::Follow)
                            .security_requirement("BearerToken")
                            .response::<200, Json<Follow>>()
                    }),
                )
                .api_route(
                    "/collect",
                    routing::post_with(collect, |op| {
                        resource_op!(op, BookmarkDocs::Collect)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkCollectResult>>()
                    }),
                )
                .api_route(
                    "/unfollow",
                    routing::post_with(unfollow, |op| {
                        resource_op!(op, BookmarkDocs::Unfollow)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkUnfollowedResponse>>()
                    }),
                )
                .api_route(
                    "/push",
                    routing::post_with(push, |op| {
                        resource_op!(op, BookmarkDocs::Push)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkPushResult>>()
                    }),
                )
                .api_route(
                    "/pull",
                    routing::post_with(pull, |op| {
                        resource_op!(op, BookmarkDocs::Pull)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkPullResult>>()
                    }),
                ),
        )
    }
}

#[expect(deprecated)]
async fn create(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<CreateBookmark>,
) -> Result<(StatusCode, Json<BookmarkResponse>), BookmarkError> {
    let response = BookmarkService::create(&state, context.project_name(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[expect(deprecated)]
async fn switch(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<SwitchBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::switch(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn merge(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<MergeBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::merge(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn list(
    context: ProjectLog,
    State(state): State<ServerState>,
    Query(params): Query<ListBookmarks>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::list(&state, context.project_name(), &params).await?,
    ))
}

#[expect(deprecated)]
async fn share(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<ShareBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::share(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn follow(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<FollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::follow(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn unfollow(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<UnfollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::unfollow(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn collect(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<CollectBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::collect(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn push(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<PushBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::push(&state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn pull(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<PullBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    Ok(Json(
        BookmarkService::pull(&state, context.project_name(), &body).await?,
    ))
}
