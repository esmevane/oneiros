use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::put(update))
        .route("/", routing::get(index))
        .route("/{name}", routing::get(show))
        .route("/{name}", routing::delete(delete))
}

async fn update(
    ticket: OneirosContext,
    Json(persona): Json<Persona>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PersonaRequests::SetPersona(persona))?))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PersonaRequests::ListPersonas(
        ListPersonasRequest,
    ))?))
}

async fn show(
    ticket: OneirosContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PersonaRequests::GetPersona(
        GetPersonaRequest { name },
    ))?))
}

async fn delete(
    ticket: OneirosContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PersonaRequests::RemovePersona(
        RemovePersonaRequest { name },
    ))?))
}
