use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct PersonaRouter;

impl PersonaRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/personas",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut persona): Json<Persona>,
) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
    persona.name = PersonaName::new(name);
    Ok((
        StatusCode::OK,
        Json(PersonaService::set(&ctx, persona).await?),
    ))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::get(&ctx, &PersonaName::new(name))?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::remove(&ctx, &PersonaName::new(name)).await?,
    ))
}
