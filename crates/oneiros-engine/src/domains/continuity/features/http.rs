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
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Json(body): Json<EmergeAgent>,
) -> Result<(StatusCode, Json<ContinuityResponse>), ContinuityError> {
    Ok((
        StatusCode::CREATED,
        Json(ContinuityService::emerge(&scope, &mailbox, &body, &DreamOverrides::default()).await?),
    ))
}

async fn recede(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::recede(
            &scope,
            &mailbox,
            &RecedeAgent::builder_v1().agent(agent).build().into(),
        )
        .await?,
    ))
}

async fn status(
    scope: Scope<AtBookmark>,
    Query(params): Query<StatusAgent>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(ContinuityService::status(&scope, &params).await?))
}

async fn wake(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::wake(
            &scope,
            &mailbox,
            &WakeAgent::builder_v1().agent(agent).build().into(),
            &overrides,
        )
        .await?,
    ))
}

async fn dream(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::dream(
            &scope,
            &mailbox,
            &DreamAgent::builder_v1().agent(agent).build().into(),
            &overrides,
        )
        .await?,
    ))
}

async fn introspect(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::introspect(
            &scope,
            &mailbox,
            &IntrospectAgent::builder_v1().agent(agent).build().into(),
            &overrides,
        )
        .await?,
    ))
}

async fn reflect(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::reflect(
            &scope,
            &mailbox,
            &ReflectAgent::builder_v1().agent(agent).build().into(),
            &overrides,
        )
        .await?,
    ))
}

async fn sense(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
    Json(body): Json<SenseContent>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    let SenseContent::V1(sensing) = body;
    let request: SenseContent = SenseContent::builder_v1()
        .agent(agent)
        .content(sensing.content)
        .build()
        .into();
    Ok(Json(
        ContinuityService::sense(&scope, &mailbox, &request, &DreamOverrides::default()).await?,
    ))
}

async fn sleep(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::sleep(
            &scope,
            &mailbox,
            &SleepAgent::builder_v1().agent(agent).build().into(),
            &overrides,
        )
        .await?,
    ))
}

async fn guidebook(
    scope: Scope<AtBookmark>,
    Path(agent): Path<AgentName>,
    Query(overrides): Query<DreamOverrides>,
) -> Result<Json<ContinuityResponse>, ContinuityError> {
    Ok(Json(
        ContinuityService::guidebook(
            &scope,
            &GuidebookAgent::builder_v1().agent(agent).build().into(),
            &overrides,
        )
        .await?,
    ))
}
