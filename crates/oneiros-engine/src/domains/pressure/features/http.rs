use axum::{Json, Router, extract::Path, routing};

use crate::*;

pub struct PressureRouter;

impl PressureRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/pressures",
            Router::new()
                .route("/", routing::get(list))
                .route("/{agent}", routing::get(get)),
        )
    }
}

async fn get(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(PressureService::get(&context, &agent)?))
}

async fn list(context: ProjectContext) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(PressureService::list(&context)?))
}
