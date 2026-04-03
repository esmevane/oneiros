use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

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
    Json(mut body): Json<SetNature>,
) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(NatureService::set(&context, &body).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListNatures>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::get(&context, &GetNature::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::remove(&context, &RemoveNature::builder().name(name).build()).await?,
    ))
}
