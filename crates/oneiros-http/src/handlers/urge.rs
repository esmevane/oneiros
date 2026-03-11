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

async fn update(ticket: OneirosContext, Json(urge): Json<Urge>) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(UrgeRequests::SetUrge(urge))?))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(UrgeRequests::ListUrges(ListUrgesRequest))?,
    ))
}

async fn show(ticket: OneirosContext, Path(name): Path<UrgeName>) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(UrgeRequests::GetUrge(
        GetUrgeRequest { name },
    ))?))
}

async fn delete(
    ticket: OneirosContext,
    Path(name): Path<UrgeName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(UrgeRequests::RemoveUrge(
        RemoveUrgeRequest { name },
    ))?))
}
