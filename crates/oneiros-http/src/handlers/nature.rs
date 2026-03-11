use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::put(set))
        .route("/", routing::get(index))
        .route("/{name}", routing::get(show))
        .route("/{name}", routing::delete(remove))
}

async fn set(ticket: OneirosContext, Json(nature): Json<Nature>) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(NatureRequests::SetNature(nature))?))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(NatureRequests::ListNatures(
        ListNaturesRequest,
    ))?))
}

async fn show(
    ticket: OneirosContext,
    Path(name): Path<NatureName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(NatureRequests::GetNature(
        GetNatureRequest { name },
    ))?))
}

async fn remove(
    ticket: OneirosContext,
    Path(name): Path<NatureName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(NatureRequests::RemoveNature(
        RemoveNatureRequest { name },
    ))?))
}
