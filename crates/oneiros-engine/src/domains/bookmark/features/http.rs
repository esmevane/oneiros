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
                    "/submit",
                    routing::post_with(submit, |op| {
                        resource_op!(op, BookmarkDocs::Submit)
                            .security_requirement("BearerToken")
                            .response::<200, Json<BookmarkSubmitResult>>()
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
    let scope = ComposeScope::new(state.config().clone()).host()?;
    let response = BookmarkService::create(&scope, &state, context.project_name(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[expect(deprecated)]
async fn switch(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<SwitchBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::switch(&scope, &state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn merge(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<MergeBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::merge(&scope, &state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn list(
    context: ProjectLog,
    State(state): State<ServerState>,
    Query(params): Query<ListBookmarks>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::list(&scope, &state, context.project_name(), &params).await?,
    ))
}

#[expect(deprecated)]
async fn share(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<ShareBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::share(&scope, &state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn follow(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<FollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::follow(&scope, &state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn unfollow(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<UnfollowBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::unfollow(&scope, &state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn collect(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<CollectBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::collect(&scope, &state, context.project_name(), &body).await?,
    ))
}

#[expect(deprecated)]
async fn submit(
    context: ProjectLog,
    State(state): State<ServerState>,
    Json(body): Json<SubmitBookmark>,
) -> Result<Json<BookmarkResponse>, BookmarkError> {
    let scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        BookmarkService::submit(&scope, &state, context.project_name(), &body).await?,
    ))
}
