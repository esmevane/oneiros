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
        .route("/", routing::post(add))
        .route("/", routing::get(index))
        .route("/{id}", routing::get(show))
}

async fn add(
    ticket: OneirosContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    Ok((
        StatusCode::CREATED,
        Json(ticket.dispatch(CognitionRequests::AddCognition(request))?),
    ))
}

async fn index(
    ticket: OneirosContext,
    Query(request): Query<ListCognitionsRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(CognitionRequests::ListCognitions(request))?,
    ))
}

async fn show(
    ticket: OneirosContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(CognitionRequests::GetCognition(
        GetCognitionRequest { id },
    ))?))
}
