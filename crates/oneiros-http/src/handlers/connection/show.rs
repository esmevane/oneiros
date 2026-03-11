use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(ConnectionRequests::GetConnection(
        GetConnectionRequest { id },
    ))?))
}
