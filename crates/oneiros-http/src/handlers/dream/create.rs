use axum::{
    Json,
    extract::{Path, Query},
};
use oneiros_model::*;

use super::collector::DreamParams;
use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent): Path<AgentName>,
    Query(params): Query<DreamParams>,
) -> Result<Json<DreamingResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_dream(DreamingRequests::Dream(DreamRequest {
            agent,
            config: params.into(),
        }))?;

    Ok(Json(response))
}
