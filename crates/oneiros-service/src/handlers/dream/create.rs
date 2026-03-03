use axum::{
    Json,
    extract::{Path, Query},
};
use oneiros_model::*;

use super::collector::DreamParams;
use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
    Query(params): Query<DreamParams>,
) -> Result<Json<DreamContext>, Error> {
    let context = ticket.service().dream(&agent_name, params.into())?;

    Ok(Json(context))
}
