use aide::axum::{ApiRouter, routing};
use axum::{Json, http::StatusCode};

use crate::*;

pub(crate) struct SliceRouter;

impl SliceRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/slices",
            ApiRouter::new().api_route(
                "/",
                routing::post_with(create, |op| {
                    resource_op!(op, SliceDocs::Create).security_requirement("BearerToken")
                }),
            ),
        )
    }
}

async fn create(
    axum::extract::State(state): axum::extract::State<ServerState>,
    scope: Scope<AtBookmark>,
    Json(body): Json<CreateSlice>,
) -> Result<(StatusCode, Json<SliceResponse>), SliceError> {
    Ok((
        StatusCode::CREATED,
        Json(SliceService::create(&scope, state.canons(), &body).await?),
    ))
}
