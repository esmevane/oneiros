use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::put(update))
        .route("/", routing::get(index))
        .route("/{name}", routing::get(show))
        .route("/{name}", routing::delete(delete))
}

async fn update(ticket: OneirosContext, Json(level): Json<Level>) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LevelRequests::SetLevel(level))?))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(LevelRequests::ListLevels(ListLevelsRequest))?,
    ))
}

async fn show(
    ticket: OneirosContext,
    Path(name): Path<LevelName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LevelRequests::GetLevel(
        GetLevelRequest { name },
    ))?))
}

async fn delete(
    ticket: OneirosContext,
    Path(name): Path<LevelName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(LevelRequests::RemoveLevel(
        RemoveLevelRequest { name },
    ))?))
}
