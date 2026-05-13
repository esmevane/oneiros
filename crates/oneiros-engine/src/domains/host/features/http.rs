use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::State, http::StatusCode};

use crate::*;

pub(crate) struct HostRouter;

impl HostRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().api_route(
            "/host",
            routing::post_with(init, |op| {
                resource_op!(op, HostDocs::Init).response::<201, Json<HostResponse>>()
            }),
        )
    }
}

async fn init(
    State(state): State<ServerState>,
    Json(body): Json<InitHost>,
) -> Result<(StatusCode, Json<HostResponse>), HostError> {
    let response = HostService::init(state.config(), state.mailbox(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
