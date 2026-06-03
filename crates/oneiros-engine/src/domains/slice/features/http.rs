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

#[expect(deprecated)]
async fn create(
    axum::extract::State(state): axum::extract::State<ServerState>,
    context: ProjectLog,
    mailbox: Mailbox,
    Json(body): Json<CreateSlice>,
) -> Result<(StatusCode, Json<SliceResponse>), SliceError> {
    let host_scope = ComposeScope::new(state.config().clone()).host()?;
    let project_scope = context.scope()?;
    Ok((
        StatusCode::CREATED,
        Json(
            SliceService::create(&host_scope, project_scope, &mailbox, state.canons(), &body)
                .await?,
        ),
    ))
}

async fn list(
    axum::extract::State(state): axum::extract::State<ServerState>,
) -> Result<Json<SliceResponse>, SliceError> {
    let host_scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(SliceService::list(&host_scope).await?))
}

async fn delete(
    axum::extract::State(state): axum::extract::State<ServerState>,
    mailbox: Mailbox,
    Path(name): Path<SliceName>,
) -> Result<Json<SliceResponse>, SliceError> {
    let host_scope = ComposeScope::new(state.config().clone()).host()?;
    Ok(Json(
        SliceService::delete(&host_scope, &mailbox, &name).await?,
    ))
}

#[expect(deprecated)]
async fn diff(
    axum::extract::State(state): axum::extract::State<ServerState>,
    context: ProjectLog,
    Json(body): Json<DiffSlice>,
) -> Result<Json<SliceResponse>, SliceError> {
    let host_scope = ComposeScope::new(state.config().clone()).host()?;
    let project_scope = context.scope()?;
    let DiffSlice::V1(req) = &body;
    Ok(Json(
        SliceService::diff(
            &host_scope,
            project_scope,
            state.canons(),
            &req.source,
            &req.target,
        )
        .await?,
    ))
}
