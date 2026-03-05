use axum::Json;
use oneiros_model::StorageResponses;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<StorageResponses>, Error> {
    let response = ticket.service().list_storage()?;

    Ok(Json(response))
}
