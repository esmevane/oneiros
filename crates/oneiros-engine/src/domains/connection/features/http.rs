use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct ConnectionRouter;

impl ConnectionRouter {
    pub fn routes(&self) -> Router<ServerState> {
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
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    entity: Option<String>,
}

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ConnectionResponse>), ConnectionError> {
    let response =
        ConnectionService::create(&context, body.from_ref, body.to_ref, body.nature).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListQuery>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(
        ConnectionService::list(&context, params.entity.as_deref()).await?,
    ))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::get(&context, &id).await?))
}

async fn remove(
    context: ProjectContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::remove(&context, &id).await?))
}
