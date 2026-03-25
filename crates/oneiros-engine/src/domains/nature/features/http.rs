use axum::{Json, Router, extract::Path, http::StatusCode, routing};

use crate::*;

pub struct NatureRouter;

impl NatureRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/natures",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<NatureName>,
    Json(mut nature): Json<Nature>,
) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
    nature.name = name;
    Ok((
        StatusCode::OK,
        Json(NatureService::set(&context, nature).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::get(&context, &name).await?))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::remove(&context, &name).await?))
}
