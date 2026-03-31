use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

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

async fn emerge(
    context: ProjectContext,
    Json(body): Json<EmergeAgent>,
) -> Result<(StatusCode, Json<ContinuityResponse>), ContinuityError> {
    Ok((
        StatusCode::CREATED,
        Json(ContinuityService::emerge(&context, &body, &DreamOverrides::default()).await?),
    ))
}

async fn recede(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::recede(&context, &RecedeAgent::builder().agent(agent).build()).await?,
    ))
}

async fn status(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::status(
        &context,
        &StatusAgent::builder().agent(agent).build(),
        &overrides,
    )?))
}

async fn wake(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::wake(
            &context,
            &WakeAgent::builder().agent(agent).build(),
            &overrides,
        )
        .await?,
    ))
}

async fn dream(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::dream(
            &context,
            &DreamAgent::builder().agent(agent).build(),
            &overrides,
        )
        .await?,
    ))
}

async fn introspect(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::introspect(
            &context,
            &IntrospectAgent::builder().agent(agent).build(),
            &overrides,
        )
        .await?,
    ))
}

async fn reflect(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::reflect(
            &context,
            &ReflectAgent::builder().agent(agent).build(),
            &overrides,
        )
        .await?,
    ))
}

async fn sense(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Json(body): Json<SenseContent>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    let selector = SenseContent::builder()
        .agent(agent)
        .content(body.content)
        .build();
    Ok(Json(
        ContinuityService::sense(&context, &selector, &DreamOverrides::default()).await?,
    ))
}

async fn sleep(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::sleep(
            &context,
            &SleepAgent::builder().agent(agent).build(),
            &overrides,
        )
        .await?,
    ))
}

async fn guidebook(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::guidebook(
        &context,
        &GuidebookAgent::builder().agent(agent).build(),
        &overrides,
    )?))
}
