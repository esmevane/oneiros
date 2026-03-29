use axum::{Json, Router, extract::Path, http::StatusCode, routing};

use crate::*;

pub struct UrgeRouter;

impl UrgeRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/urges",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<UrgeName>,
    Json(mut urge): Json<Urge>,
) -> Result<(StatusCode, Json<UrgeResponse>), UrgeError> {
    urge.name = name;
    Ok((
        StatusCode::OK,
        Json(UrgeService::set(&context, urge).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::list(&context)?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<UrgeName>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::get(&context, &name)?))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<UrgeName>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::remove(&context, &name).await?))
}
