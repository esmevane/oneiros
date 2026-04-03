use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct ActorRouter;

impl ActorRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/actors",
            Router::<ServerState>::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show)),
        )
    }
}

async fn create(
    context: SystemContext,
    Json(body): Json<CreateActor>,
) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
    let response = ActorService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: SystemContext,
    Query(params): Query<ListActors>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::list(&context, &params).await?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<ActorId>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(
        ActorService::get(&context, &GetActor::builder().id(id).build()).await?,
    ))
}
