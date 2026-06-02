use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::Path, http::StatusCode};

use crate::*;

pub(crate) struct SliceRouter;

impl SliceRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/slices",
            ApiRouter::new()
                .api_route(
                    "/diff",
                    routing::post_with(diff, |op| {
                        resource_op!(op, SliceDocs::Diff).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/bookmark",
                    routing::post_with(bookmark, |op| {
                        resource_op!(op, SliceDocs::Bookmark).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/",
                    routing::post_with(create, |op| {
                        resource_op!(op, SliceDocs::Create).security_requirement("BearerToken")
                    })
                    .get_with(list, |op| {
                        resource_op!(op, SliceDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::delete_with(delete, |op| {
                        resource_op!(op, SliceDocs::Delete).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn create(
    axum::extract::State(state): axum::extract::State<ServerState>,
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Json(body): Json<CreateSlice>,
) -> Result<(StatusCode, Json<SliceResponse>), SliceError> {
    Ok((
        StatusCode::CREATED,
        Json(SliceService::create(&scope, &mailbox, state.canons(), &body).await?),
    ))
}

async fn list(
    scope: Scope<AtBookmark>,
) -> Result<Json<SliceResponse>, SliceError> {
    Ok(Json(SliceService::list(&scope).await?))
}

async fn delete(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<SliceName>,
) -> Result<Json<SliceResponse>, SliceError> {
    Ok(Json(SliceService::delete(&scope, &mailbox, &name).await?))
}

async fn diff(
    axum::extract::State(state): axum::extract::State<ServerState>,
    scope: Scope<AtBookmark>,
    Json(body): Json<DiffSlice>,
) -> Result<Json<SliceResponse>, SliceError> {
    let DiffSlice::V1(req) = &body;
    Ok(Json(
        SliceService::diff(&scope, state.canons(), &req.source, &req.target).await?,
    ))
}

async fn bookmark(
    axum::extract::State(state): axum::extract::State<ServerState>,
    scope: Scope<AtBookmark>,
    Json(body): Json<BookmarkSlice>,
) -> Result<Json<SliceResponse>, SliceError> {
    Ok(Json(
        SliceService::bookmark(&state, &scope, &body).await?,
    ))
}
