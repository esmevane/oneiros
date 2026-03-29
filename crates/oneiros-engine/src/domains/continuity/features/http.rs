use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct ContinuityRouter;

impl ContinuityRouter {
    pub fn routes(&self) -> Router<ServerState> {
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

#[derive(Debug, Deserialize)]
struct EmergeBody {
    name: AgentName,
    persona: PersonaName,
    description: Description,
}

#[derive(Debug, Deserialize)]
struct SenseBody {
    content: Content,
}

async fn emerge(
    context: ProjectContext,
    Json(body): Json<EmergeBody>,
) -> Result<(StatusCode, Json<ContinuityResponse>), ContinuityError> {
    Ok((
        StatusCode::CREATED,
        Json(
            ContinuityService::emerge(
                &context,
                body.name,
                body.persona,
                body.description,
                &DreamOverrides::default(),
            )
            .await?,
        ),
    ))
}

async fn recede(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::recede(&context, &agent).await?))
}

async fn status(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::status(
        &context, &agent, &overrides,
    )?))
}

async fn wake(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::wake(&context, &agent, &overrides).await?,
    ))
}

async fn dream(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::dream(&context, &agent, &overrides).await?,
    ))
}

async fn introspect(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::introspect(&context, &agent, &overrides).await?,
    ))
}

async fn reflect(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::reflect(&context, &agent, &overrides).await?,
    ))
}

async fn sense(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Json(body): Json<SenseBody>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::sense(&context, &agent, &body.content, &DreamOverrides::default())
            .await?,
    ))
}

async fn sleep(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::sleep(&context, &agent, &overrides).await?,
    ))
}

async fn guidebook(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::guidebook(
        &context, &agent, &overrides,
    )?))
}
