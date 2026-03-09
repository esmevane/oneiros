use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<ConnectionResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_connection(ConnectionRequests::GetConnection(GetConnectionRequest {
            id,
        }))?;

    Ok(Json(response))
}
