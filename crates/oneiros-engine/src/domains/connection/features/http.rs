use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::contexts::ProjectContext;

use super::super::errors::ConnectionError;
use super::super::responses::ConnectionResponse;
use super::super::service::ConnectionService;

pub const PATH: &str = "/connections";

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list).post(create))
        .route("/{id}", routing::get(show).delete(remove))
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    from_entity: String,
    to_entity: String,
    nature: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    entity: Option<String>,
}

async fn create(
    State(ctx): State<ProjectContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ConnectionResponse>), ConnectionError> {
    let response = ConnectionService::create(
        &ctx,
        body.from_entity,
        body.to_entity,
        body.nature,
        body.description,
    )?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(ctx): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::list(
        &ctx,
        params.entity.as_deref(),
    )?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::get(&ctx, &id)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::remove(&ctx, &id)?))
}
