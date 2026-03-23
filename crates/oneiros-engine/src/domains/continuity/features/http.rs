use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct ContinuityRouter;

impl ContinuityRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/continuity",
            Router::new()
                .route("/", routing::post(emerge))
                .route("/{agent}", routing::get(status).delete(recede))
                .route("/{agent}/wake", routing::post(wake))
                .route("/{agent}/dream", routing::post(dream))
                .route("/{agent}/introspect", routing::post(introspect))
                .route("/{agent}/reflect", routing::post(reflect))
                .route("/{agent}/sense", routing::post(sense))
                .route("/{agent}/sleep", routing::post(sleep))
                .route("/{agent}/guidebook", routing::get(guidebook)),
        )
    }
}

async fn emerge(
    State(context): State<ProjectContext>,
    Json(body): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<ContinuityResponse>), ContinuityError> {
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let persona = body.get("persona").and_then(|v| v.as_str()).unwrap_or("");
    let description = body
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    Ok((
        StatusCode::CREATED,
        Json(ContinuityService::emerge(
            &context,
            AgentName::new(name),
            PersonaName::new(persona),
            Description::new(description),
            &DreamOverrides::default(),
        )?),
    ))
}

async fn recede(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::recede(
        &context,
        &AgentName::new(&agent),
    )?))
}

async fn status(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::status(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}

async fn wake(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::wake(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}

async fn dream(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::dream(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}

async fn introspect(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::introspect(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}

async fn reflect(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::reflect(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}

async fn sense(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or("");
    let content = Content::new(content_str);
    Ok(Json(ContinuityService::sense(
        &context,
        &AgentName::new(&agent),
        &content,
        &DreamOverrides::default(),
    )?))
}

async fn sleep(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::sleep(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}

async fn guidebook(
    State(context): State<ProjectContext>,
    Path(agent): Path<String>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::guidebook(
        &context,
        &AgentName::new(&agent),
        &overrides,
    )?))
}
