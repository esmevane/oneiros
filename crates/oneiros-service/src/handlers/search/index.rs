use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct SearchParams {
    pub q: String,
    pub agent: Option<AgentName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchResults>, Error> {
    let results = ticket.service().search(&params.q, params.agent.as_ref())?;

    Ok(Json(results))
}
