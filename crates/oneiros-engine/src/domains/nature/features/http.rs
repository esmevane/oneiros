use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct NatureRouter;

impl NatureRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/natures",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut nature): Json<Nature>,
) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
    nature.name = NatureName::new(name);
    Ok((
        StatusCode::OK,
        Json(NatureService::set(&ctx, nature).await?),
    ))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::get(&ctx, &NatureName::new(name))?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::remove(&ctx, &NatureName::new(name)).await?,
    ))
}
