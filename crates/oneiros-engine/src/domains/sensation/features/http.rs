use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub(crate) struct SensationRouter;

impl SensationRouter {
    pub(crate) fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/sensations",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<SensationName>,
    Json(mut body): Json<SetSensation>,
) -> Result<(StatusCode, Json<SensationResponse>), SensationError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(SensationService::set(&context, &body).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListSensations>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<SensationName>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(
        SensationService::get(&context, &GetSensation::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<SensationName>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(
        SensationService::remove(&context, &RemoveSensation::builder().name(name).build()).await?,
    ))
}
