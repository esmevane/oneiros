use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::*;

pub(crate) struct RemoteRouter;

impl RemoteRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::<ServerState>::new().nest(
            "/remotes",
            ApiRouter::<ServerState>::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, RemoteDocs::List).response::<200, Json<RemotesResponse>>()
                    })
                    .post_with(add, |op| {
                        resource_op!(op, RemoteDocs::Add)
                            .response::<201, Json<RemoteAddedResponse>>()
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::delete_with(remove, |op| {
                        resource_op!(op, RemoteDocs::Remove)
                            .response::<200, Json<RemoteRemovedResponse>>()
                    }),
                )
                .api_route(
                    "/{name}/bookmarks",
                    routing::get_with(bookmarks, |op| {
                        resource_op!(op, RemoteDocs::Bookmarks)
                            .response::<200, Json<RemoteBookmarkListResponse>>()
                    }),
                ),
        )
    }
}

async fn add(
    State(state): State<ServerState>,
    Json(body): Json<AddRemote>,
) -> Result<(StatusCode, Json<RemoteResponse>), RemoteError> {
    let response = RemoteService::add(&state, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListRemotes>,
) -> Result<Json<RemoteResponse>, RemoteError> {
    Ok(Json(RemoteService::list(&scope, &params).await?))
}

async fn remove(
    scope: Scope<AtHost>,
    Path(name): Path<RemoteName>,
    State(state): State<ServerState>,
) -> Result<Json<RemoteResponse>, RemoteError> {
    Ok(Json(
        RemoteService::remove(
            &scope,
            state.mailbox(),
            &RemoveRemote::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}

async fn bookmarks(
    State(state): State<ServerState>,
    Path(name): Path<RemoteName>,
) -> Result<Json<RemoteResponse>, RemoteError> {
    Ok(Json(RemoteService::bookmarks(&state, &name).await?))
}
