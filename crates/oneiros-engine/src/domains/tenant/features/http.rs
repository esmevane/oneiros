use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use serde::Deserialize;

use crate::*;

pub struct TenantRouter;

impl TenantRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/tenants",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show)),
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
) -> Result<(StatusCode, Json<TenantResponse>), TenantError> {
    let response = TenantService::create(&context, TenantName::new(body.name)).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: SystemContext) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(TenantService::list(&context).await?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<TenantId>,
) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(TenantService::get(&context, &id).await?))
}
