use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::contexts::ProjectContext;

use super::super::errors::NatureError;
use super::super::model::Nature;
use super::super::responses::NatureResponse;
use super::super::service::NatureService;

pub const PATH: &str = "/natures";

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list))
        .route("/{name}", routing::put(set).get(show).delete(remove))
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut nature): Json<Nature>,
) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
    nature.name = name;
    Ok((StatusCode::OK, Json(NatureService::set(&ctx, nature)?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::get(&ctx, &name)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::remove(&ctx, &name)?))
}
