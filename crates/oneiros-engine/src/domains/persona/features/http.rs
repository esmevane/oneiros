use axum::{Json, Router, extract::Path, http::StatusCode, routing};

use crate::*;

pub struct PersonaRouter;

impl PersonaRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/personas",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<PersonaName>,
    Json(mut body): Json<SetPersona>,
) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(PersonaService::set(&context, &body).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::get(&context, &GetPersona::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::remove(&context, &RemovePersona::builder().name(name).build()).await?,
    ))
}
