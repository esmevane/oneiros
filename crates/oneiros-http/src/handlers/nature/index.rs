use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<NatureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_nature(NatureRequests::ListNatures(ListNaturesRequest))?;

    Ok(Json(response))
}
