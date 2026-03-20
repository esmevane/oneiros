use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct UrgeRouter;

impl UrgeRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/urges",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut urge): Json<Urge>,
) -> Result<(StatusCode, Json<UrgeResponse>), UrgeError> {
    urge.name = UrgeName::new(name);
    Ok((StatusCode::OK, Json(UrgeService::set(&ctx, urge)?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::get(&ctx, &UrgeName::new(name))?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::remove(&ctx, &UrgeName::new(name))?))
}
