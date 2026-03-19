use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct ConnectionRouter;

impl ConnectionRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/connections",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show).delete(remove)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    from_ref: String,
    to_ref: String,
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
        body.from_ref,
        body.to_ref,
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
