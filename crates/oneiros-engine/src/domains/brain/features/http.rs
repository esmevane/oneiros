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
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Json(body): Json<CreateBrain>,
) -> Result<(StatusCode, Json<BrainResponse>), BrainError> {
    let response = BrainService::create(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListBrains>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(BrainService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtHost>,
    Path(key): Path<ResourceKey<BrainName>>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(
        BrainService::get(&scope, &GetBrain::builder_v1().key(key).build().into()).await?,
    ))
}
