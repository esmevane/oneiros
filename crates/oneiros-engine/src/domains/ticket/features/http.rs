use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct TicketRouter;

impl TicketRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/tickets",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| resource_op!(op, TicketDocs::List)).post_with(
                        create,
                        |op| {
                            resource_op!(op, TicketDocs::Create)
                                .response::<201, Json<TicketResponse>>()
                        },
                    ),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| resource_op!(op, TicketDocs::Show)),
                )
                .api_route(
                    "/validate",
                    routing::post_with(validate, |op| resource_op!(op, TicketDocs::Validate)),
                ),
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
    Path(key): Path<ResourceKey<TicketId>>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(
        TicketService::get(&context, &GetTicket::builder_v1().key(key).build().into()).await?,
    ))
}

async fn validate(
    context: SystemContext,
    Json(body): Json<ValidateTicket>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::validate(&context, &body).await?))
}
