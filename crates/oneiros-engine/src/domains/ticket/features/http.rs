use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub(crate) struct TicketRouter;

impl TicketRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/tickets",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, TicketDocs::List).response::<200, Json<TicketsResponse>>()
                    })
                    .post_with(create, |op| {
                        resource_op!(op, TicketDocs::Create)
                            .response::<201, Json<TicketCreatedResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, TicketDocs::Show)
                            .input::<IdPathParam<TicketId>>()
                            .response::<200, Json<TicketFoundResponse>>()
                    })
                    .delete_with(revoke, |op| {
                        resource_op!(op, TicketDocs::Revoke)
                            .response::<200, Json<TicketRevokedResponse>>()
                    }),
                )
                .api_route(
                    "/validate",
                    routing::post_with(validate, |op| {
                        resource_op!(op, TicketDocs::Validate)
                            .response::<200, Json<TicketValidatedResponse>>()
                    }),
                ),
        )
    }
}

async fn create(
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Json(body): Json<CreateTicket>,
) -> Result<(StatusCode, Json<TicketResponse>), TicketError> {
    let response = TicketService::create(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListTickets>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtHost>,
    Path(key): Path<ResourceKey<TicketId>>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(
        TicketService::get(&scope, &GetTicket::builder_v1().key(key).build().into()).await?,
    ))
}

async fn validate(
    scope: Scope<AtHost>,
    Json(body): Json<ValidateTicket>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::validate(&scope, &body).await?))
}

async fn revoke(
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Path(key): Path<ResourceKey<TicketId>>,
) -> Result<Json<TicketResponse>, TicketError> {
    let id = key.resolve()?;
    Ok(Json(
        TicketService::revoke(
            &scope,
            &mailbox,
            &RevokeTicket::builder_v1().ticket_id(id).build().into(),
        )
        .await?,
    ))
}
