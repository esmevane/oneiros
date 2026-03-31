use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

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

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateConnection>,
) -> Result<(StatusCode, Json<ConnectionResponse>), ConnectionError> {
    let response = ConnectionService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListConnections>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(
        ConnectionService::get(&context, &GetConnection::builder().id(id).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(
        ConnectionService::remove(&context, &RemoveConnection::builder().id(id).build()).await?,
    ))
}
