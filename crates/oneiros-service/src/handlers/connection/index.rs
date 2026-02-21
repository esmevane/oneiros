use axum::{Json, extract::Query};
use oneiros_model::{Connection, ConnectionId, Identity, Key, Link, NatureName};
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub nature: Option<NatureName>,
    pub link: Option<Link>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Identity<ConnectionId, Connection>>>, Error> {
    let connections = match (params.nature, params.link) {
        (Some(nature), Some(link)) => {
            ticket
                .db
                .get_nature(&nature)?
                .ok_or(NotFound::Nature(Key::Id(nature.clone())))?;

            ticket
                .db
                .list_connections_by_link(&link)?
                .into_iter()
                .filter(|c| c.nature == nature)
                .collect()
        }
        (Some(nature), None) => {
            ticket
                .db
                .get_nature(&nature)?
                .ok_or(NotFound::Nature(Key::Id(nature.clone())))?;

            ticket.db.list_connections_by_nature(&nature)?
        }
        (None, Some(link)) => ticket.db.list_connections_by_link(&link)?,
        (None, None) => ticket.db.list_connections()?,
    };

    Ok(Json(connections))
}
