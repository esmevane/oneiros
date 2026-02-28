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
    let connections = match (params.nature, params.entity_ref) {
        (Some(nature), Some(entity_ref)) => {
            ticket
                .db
                .get_nature(&nature)?
                .ok_or(NotFound::Nature(nature.clone()))?;

            ticket
                .db
                .list_connections_by_ref(entity_ref.inner())?
                .into_iter()
                .filter(|c| c.nature == nature)
                .collect()
        }
        (Some(nature), None) => {
            ticket
                .db
                .get_nature(&nature)?
                .ok_or(NotFound::Nature(nature.clone()))?;

            ticket.db.list_connections_by_nature(&nature)?
        }
        (None, Some(entity_ref)) => ticket.db.list_connections_by_ref(entity_ref.inner())?,
        (None, None) => ticket.db.list_connections()?,
    };

    Ok(Json(connections))
}
