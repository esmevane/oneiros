use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct MemoryRouter;

impl MemoryRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/memories",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, MemoryDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(add, |op| {
                        resource_op!(op, MemoryDocs::Add)
                            .security_requirement("BearerToken")
                            .response::<201, Json<MemoryResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, MemoryDocs::Show).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn add(
    context: ProjectLog,
    Json(body): Json<AddMemory>,
) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
    let response = MemoryService::add(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectLog,
    Query(params): Query<ListMemories>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::list(&context, &params).await?))
}

async fn show(
    context: ProjectLog,
    Path(key): Path<ResourceKey<MemoryId>>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(
        MemoryService::get(&context, &GetMemory::builder_v1().key(key).build().into()).await?,
    ))
}
