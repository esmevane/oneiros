use aide::axum::{ApiRouter, routing};
use axum::{Json, http::StatusCode};

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
    context: HostLog,
    Json(body): Json<InitSystem>,
) -> Result<(StatusCode, Json<SystemResponse>), SystemError> {
    let response = SystemService::init(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
