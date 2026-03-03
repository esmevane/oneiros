use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub nature: Option<NatureName>,
    pub entity_ref: Option<RefToken>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Connection>>, Error> {
    let connections = ticket.service().list_connections(
        params.nature,
        params.entity_ref.as_ref().map(RefToken::inner),
    )?;

    Ok(Json(connections))
}
