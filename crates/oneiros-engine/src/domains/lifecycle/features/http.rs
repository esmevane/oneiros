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
            .route("/wake/{agent}", routing::post(wake))
            .route("/dream/{agent}", routing::post(dream))
            .route("/introspect/{agent}", routing::post(introspect))
            .route("/reflect/{agent}", routing::post(reflect))
            .route("/sense/{agent}", routing::post(sense))
            .route("/sleep/{agent}", routing::post(sleep))
            .route("/guidebook/{agent}", routing::get(guidebook))
    }
}

async fn wake(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::wake(
        &context,
        &AgentName::new(&agent),
    )?))
}

async fn dream(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::dream(
        &context,
        &AgentName::new(&agent),
    )?))
}

async fn introspect(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::introspect(
        &context,
        &AgentName::new(&agent),
    )?))
}

async fn reflect(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::reflect(
        &context,
        &AgentName::new(&agent),
    )?))
}

async fn sense(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or("");
    let content = Content::new(content_str);
    Ok(Json(LifecycleService::sense(
        &context,
        &AgentName::new(&agent),
        &content,
    )?))
}

async fn sleep(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::sleep(
        &context,
        &AgentName::new(&agent),
    )?))
}

async fn guidebook(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<LifecycleResponse>, LifecycleError> {
    Ok(Json(LifecycleService::guidebook(
        &context,
        &AgentName::new(&agent),
    )?))
}
