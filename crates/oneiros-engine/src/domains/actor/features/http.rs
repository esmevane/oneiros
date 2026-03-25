use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use serde::Deserialize;

use crate::*;

pub struct ActorRouter;

impl ActorRouter {
    pub fn routes(&self) -> Router<ServerState> {
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
    context: SystemContext,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
    let response =
        ActorService::create(&context, body.tenant_id, ActorName::new(body.name)).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: SystemContext) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::list(&context).await?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<ActorId>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::get(&context, id).await?))
}
