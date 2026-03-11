use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::put(set))
        .route("/", routing::get(index))
        .route("/{name}", routing::get(show))
        .route("/{name}", routing::delete(remove))
}

async fn set(
    ticket: OneirosContext,
    Json(sensation): Json<Sensation>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(SensationRequests::SetSensation(sensation))?,
    ))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(SensationRequests::ListSensations(
        ListSensationsRequest,
    ))?))
}

async fn show(
    ticket: OneirosContext,
    Path(name): Path<SensationName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(SensationRequests::GetSensation(
        GetSensationRequest { name },
    ))?))
}

async fn remove(
    ticket: OneirosContext,
    Path(name): Path<SensationName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(
        SensationRequests::RemoveSensation(RemoveSensationRequest { name }),
    )?))
}
