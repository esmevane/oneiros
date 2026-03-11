use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::post(create))
        .route("/", routing::get(index))
        .route("/{id}", routing::get(show))
        .route("/{id}", routing::delete(remove))
}

async fn create(
    ticket: OneirosContext,
    Json(request): Json<CreateConnectionRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    Ok((
        StatusCode::CREATED,
        Json(ticket.dispatch(ConnectionRequests::CreateConnection(request))?),
    ))
}

async fn index(
    ticket: OneirosContext,
    Query(request): Query<ListConnectionsRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(ConnectionRequests::ListConnections(request))?,
    ))
}

async fn show(
    ticket: OneirosContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(ConnectionRequests::GetConnection(
        GetConnectionRequest { id },
    ))?))
}

async fn remove(
    ticket: OneirosContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(
        ConnectionRequests::RemoveConnection(RemoveConnectionRequest { id }),
    )?))
}
