use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::State, http::StatusCode};

use crate::*;

pub struct SystemRouter;

impl SystemRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().api_route(
            "/system",
            routing::post_with(init, |op| {
                resource_op!(op, SystemDocs::Init).response::<201, Json<SystemResponse>>()
            }),
        )
    }
}

async fn init(
    State(state): State<ServerState>,
    Json(body): Json<InitSystem>,
) -> Result<(StatusCode, Json<SystemResponse>), SystemError> {
    let response = SystemService::init(state.config(), state.mailbox(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
