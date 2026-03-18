use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::contexts::ProjectContext;

use super::super::errors::PersonaError;
use super::super::model::Persona;
use super::super::responses::PersonaResponse;
use super::super::service::PersonaService;

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list))
        .route("/{name}", routing::put(set).get(show).delete(remove))
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut persona): Json<Persona>,
) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
    persona.name = name;
    Ok((StatusCode::OK, Json(PersonaService::set(&ctx, persona)?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::get(&ctx, &name)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::remove(&ctx, &name)?))
}
