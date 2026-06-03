use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub(crate) struct MemoryRouter;

impl MemoryRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/memories",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, MemoryDocs::List)
                            .security_requirement("BearerToken")
                            .response::<200, Json<MemoriesResponse>>()
                    })
                    .post_with(add, |op| {
                        resource_op!(op, MemoryDocs::Add)
                            .security_requirement("BearerToken")
                            .response::<201, Json<MemoryAddedResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, MemoryDocs::Show)
                            .security_requirement("BearerToken")
                            .input::<IdPathParam<MemoryId>>()
                            .response::<200, Json<MemoryDetailsResponse>>()
                    }),
                ),
        )
    }
}

async fn add(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Json(body): Json<AddMemory>,
) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
    let response = MemoryService::add(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    axum::extract::State(state): axum::extract::State<ServerState>,
    scope: Scope<AtBookmark>,
    Query(params): Query<ListMemories>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    let ListMemories::V1(listing) = &params;
    if let Some(source) = listing.lens.as_deref() {
        return Ok(Json(
            MemoryLens::new(&scope, state.canons())
                .list(source, &listing.filters)
                .await?,
        ));
    }
    Ok(Json(MemoryService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(key): Path<ResourceKey<MemoryId>>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(
        MemoryService::get(&scope, &GetMemory::builder_v1().key(key).build().into()).await?,
    ))
}
