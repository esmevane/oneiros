use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct EventParams {
    pub after: Option<u64>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<EventParams>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = ticket.service().read_events(params.after)?;

    Ok(Json(events))
}
