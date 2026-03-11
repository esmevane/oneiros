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
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(DreamingRequests::Dream(
        DreamRequest {
            agent,
            config: params.into(),
        },
    ))?))
}
