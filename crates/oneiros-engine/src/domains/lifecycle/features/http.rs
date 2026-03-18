use axum::{
    Json, Router,
    extract::{Path, State},
    routing,
};

use crate::*;

pub struct LifecycleRouter;

impl LifecycleRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new()
            .route("/dream/{agent}", routing::post(dream))
            .route("/introspect/{agent}", routing::post(introspect))
            .route("/reflect/{agent}", routing::post(reflect))
            .route("/sense/{agent}", routing::post(sense))
            .route("/sleep/{agent}", routing::post(sleep))
    }
}

async fn dream(
    State(ctx): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::dream(&ctx, &agent)?))
}

async fn introspect(
    State(ctx): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::introspect(&ctx, &agent)?))
}

async fn reflect(
    State(ctx): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::reflect(&ctx, &agent)?))
}

async fn sense(
    State(ctx): State<ProjectContext>,
    Path(agent): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Json(LifecycleService::sense(&ctx, &agent, content)?))
}

async fn sleep(
    State(ctx): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::sleep(&ctx, &agent)?))
}
