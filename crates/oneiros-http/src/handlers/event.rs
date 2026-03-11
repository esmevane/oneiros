use axum::extract::DefaultBodyLimit;
use axum::{
    Json, Router,
    extract::{Path, Query},
    routing,
};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::get(index))
        .route("/import", routing::post(import))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .route("/replay", routing::post(replay))
        .route("/{id}", routing::get(show))
}

async fn index(
    ticket: OneirosContext,
    Query(request): Query<ListEventsRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::ListEvents(request))?))
}

async fn import(
    ticket: OneirosContext,
    Json(events): Json<Vec<ImportEvent>>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::ImportEvents(
        ImportEventsRequest { events },
    ))?))
}

async fn replay(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::ReplayEvents(
        ReplayEventsRequest,
    ))?))
}

async fn show(ticket: OneirosContext, Path(id): Path<EventId>) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(EventRequests::GetEvent(
        GetEventRequest { id },
    ))?))
}
