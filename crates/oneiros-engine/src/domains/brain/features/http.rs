use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use serde::Deserialize;

use crate::*;

pub struct BrainRouter;

impl BrainRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/brains",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{name}", routing::get(show)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    name: String,
}

async fn create(
    context: SystemContext,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<BrainResponse>), BrainError> {
    let response = BrainService::create(&context, BrainName::new(body.name)).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: SystemContext) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(BrainService::list(&context).await?))
}

async fn show(
    context: SystemContext,
    Path(name): Path<String>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(
        BrainService::get(&context, &BrainName::new(name)).await?,
    ))
}
