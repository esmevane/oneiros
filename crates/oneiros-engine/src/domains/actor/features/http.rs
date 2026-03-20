use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct ActorRouter;

impl ActorRouter {
    pub fn routes(&self) -> Router<SystemContext> {
        Router::new().nest(
            "/actors",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    tenant_id: TenantId,
    name: String,
}

async fn create(
    State(ctx): State<SystemContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
    let response = ActorService::create(&ctx, body.tenant_id, ActorName::new(body.name))?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<SystemContext>) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::list(&ctx)?))
}

async fn show(
    State(ctx): State<SystemContext>,
    Path(id): Path<ActorId>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::get(&ctx, &id)?))
}
