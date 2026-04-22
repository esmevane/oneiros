use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct AgentRouter;

impl AgentRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/agents",
            ApiRouter::<ServerState>::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, AgentDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(create, |op| {
                        resource_op!(op, AgentDocs::Create)
                            .security_requirement("BearerToken")
                            .response::<201, Json<AgentResponse>>()
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::get_with(show, |op| {
                        resource_op!(op, AgentDocs::Show).security_requirement("BearerToken")
                    })
                    .put_with(update, |op| {
                        resource_op!(op, AgentDocs::Update).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, AgentDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateAgent>,
) -> Result<(StatusCode, Json<AgentResponse>), AgentError> {
    let response = AgentService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListAgents>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(key): Path<ResourceKey<AgentName>>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::get(&context, &GetAgent::builder().key(key).build()).await?,
    ))
}

async fn update(
    context: ProjectContext,
    Path(_): Path<AgentName>,
    Json(body): Json<UpdateAgent>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::update(&context, &body).await?))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::remove(&context, &RemoveAgent::builder().name(name).build()).await?,
    ))
}
