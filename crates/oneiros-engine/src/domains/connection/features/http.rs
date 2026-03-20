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
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    entity: Option<String>,
}

async fn create(
    State(context): State<ProjectContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ConnectionResponse>), ConnectionError> {
    let response = ConnectionService::create(&context, body.from_ref, body.to_ref, body.nature)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(context): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::list(
        &context,
        params.entity.as_deref(),
    )?))
}

async fn show(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    let id: ConnectionId = id
        .parse()
        .map_err(|e: IdParseError| ConnectionError::Database(e.into()))?;
    Ok(Json(ConnectionService::get(&context, &id)?))
}

async fn remove(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    let id: ConnectionId = id
        .parse()
        .map_err(|e: IdParseError| ConnectionError::Database(e.into()))?;
    Ok(Json(ConnectionService::remove(&context, &id)?))
}
