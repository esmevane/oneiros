use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct TenantRouter;

impl TenantRouter {
    pub fn routes(&self) -> Router<SystemContext> {
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
    State(ctx): State<SystemContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<TenantResponse>), TenantError> {
    let response = TenantService::create(&ctx, body.name)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<SystemContext>) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(TenantService::list(&ctx)?))
}

async fn show(
    State(ctx): State<SystemContext>,
    Path(id): Path<String>,
) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(TenantService::get(&ctx, &id)?))
}
