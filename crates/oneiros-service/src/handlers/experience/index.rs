use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub sensation: Option<SensationName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Experience>>, Error> {
    let experiences = ticket
        .service()
        .list_experiences(params.agent, params.sensation)?;

    Ok(Json(experiences))
}
