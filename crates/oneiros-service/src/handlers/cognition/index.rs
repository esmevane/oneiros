use axum::{Json, extract::Query};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub texture: Option<TextureName>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Cognition>>, Error> {
    let cognitions = ticket
        .service()
        .list_cognitions(params.agent, params.texture)?;

    Ok(Json(cognitions))
}
