use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct TicketRouter;

impl TicketRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/tickets",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show))
                .route("/validate", routing::post(validate)),
        )
    }
}

async fn create(
    context: SystemContext,
    Json(body): Json<CreateTicket>,
) -> Result<(StatusCode, Json<TicketResponse>), TicketError> {
    let response = TicketService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: SystemContext,
    Query(params): Query<ListTickets>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::list(&context, &params).await?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<TicketId>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(
        TicketService::get(&context, &GetTicket::builder().id(id).build()).await?,
    ))
}

async fn validate(
    context: SystemContext,
    Json(body): Json<ValidateTicket>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::validate(&context, &body).await?))
}
