use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
struct CreateBody {
    actor_id: ActorId,
    brain_name: String,
}

#[derive(Debug, Deserialize)]
struct ValidateBody {
    token: String,
}

async fn create(
    context: SystemContext,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<TicketResponse>), TicketError> {
    let response =
        TicketService::create(&context, body.actor_id, BrainName::new(body.brain_name)).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: SystemContext) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::list(&context)?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<TicketId>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::get(&context, &id)?))
}

async fn validate(
    context: SystemContext,
    Json(body): Json<ValidateBody>,
) -> Result<Json<TicketResponse>, TicketError> {
    Ok(Json(TicketService::validate(&context, &body.token)?))
}
