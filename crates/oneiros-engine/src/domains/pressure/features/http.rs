use axum::{Json, Router, extract::Path, routing};

use crate::*;

pub(crate) struct PressureRouter;

impl PressureRouter {
    pub(crate) fn routes(&self) -> Router<ServerState> {
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
    Ok(Json(
        PressureService::get(&context, &GetPressure::builder().agent(agent).build()).await?,
    ))
}

async fn list(context: ProjectContext) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(PressureService::list(&context).await?))
}
