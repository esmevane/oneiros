use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::get(index))
        .route("/{agent}", routing::get(show))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PressureRequests::ListPressures(
        ListPressuresRequest,
    ))?))
}

async fn show(
    ticket: OneirosContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PressureRequests::GetPressure(
        GetPressureRequest { agent },
    ))?))
}
