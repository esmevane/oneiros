use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct BrainRouter;

impl BrainRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/brains",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| resource_op!(op, BrainDocs::List)).post_with(
                        create,
                        |op| {
                            resource_op!(op, BrainDocs::Create)
                                .response::<201, Json<BrainResponse>>()
                        },
                    ),
                )
                .api_route(
                    "/{name}",
                    routing::get_with(show, |op| resource_op!(op, BrainDocs::Show)),
                ),
        )
    }
}

async fn create(
    context: HostLog,
    Json(body): Json<CreateBrain>,
) -> Result<(StatusCode, Json<BrainResponse>), BrainError> {
    let response = BrainService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: HostLog,
    Query(params): Query<ListBrains>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(BrainService::list(&context, &params).await?))
}

async fn show(
    context: HostLog,
    Path(key): Path<ResourceKey<BrainName>>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(
        BrainService::get(&context, &GetBrain::builder_v1().key(key).build().into()).await?,
    ))
}
