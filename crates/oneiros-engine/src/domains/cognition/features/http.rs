use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct CognitionRouter;

impl CognitionRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/cognitions",
            Router::new()
                .route("/", routing::get(list).post(add))
                .route("/{id}", routing::get(show)),
        )
    }
}

async fn add(
    context: ProjectContext,
    Json(body): Json<AddCognition>,
) -> Result<(StatusCode, Json<CognitionResponse>), CognitionError> {
    let response = CognitionService::add(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListCognitions>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(
        CognitionService::get(&context, &GetCognition::builder().id(id).build()).await?,
    ))
}
