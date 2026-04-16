use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct ContinuityRouter;

impl ContinuityRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/continuity",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(status, |op| {
                        resource_op!(op, ContinuityDocs::Status).security_requirement("BearerToken")
                    })
                    .post_with(emerge, |op| {
                        resource_op!(op, ContinuityDocs::Emerge)
                            .security_requirement("BearerToken")
                            .response::<201, Json<ContinuityResponse>>()
                    }),
                )
                .api_route(
                    "/{agent}",
                    routing::delete_with(recede, |op| {
                        resource_op!(op, ContinuityDocs::Recede).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/wake",
                    routing::post_with(wake, |op| {
                        resource_op!(op, ContinuityDocs::Wake).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/dream",
                    routing::post_with(dream, |op| {
                        resource_op!(op, ContinuityDocs::Dream).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/introspect",
                    routing::post_with(introspect, |op| {
                        resource_op!(op, ContinuityDocs::Introspect)
                            .security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/reflect",
                    routing::post_with(reflect, |op| {
                        resource_op!(op, ContinuityDocs::Reflect)
                            .security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/sense",
                    routing::post_with(sense, |op| {
                        resource_op!(op, ContinuityDocs::Sense).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/sleep",
                    routing::post_with(sleep, |op| {
                        resource_op!(op, ContinuityDocs::Sleep).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}/guidebook",
                    routing::get_with(guidebook, |op| {
                        resource_op!(op, ContinuityDocs::Guidebook)
                            .security_requirement("BearerToken")
                    }),
                ),
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
    Query(params): Query<StatusAgent>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::status(&context, &params)?))
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
