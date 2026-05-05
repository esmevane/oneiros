use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct CognitionRouter;

impl CognitionRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/cognitions",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, CognitionDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(add, |op| {
                        resource_op!(op, CognitionDocs::Add)
                            .security_requirement("BearerToken")
                            .response::<201, Json<CognitionResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, CognitionDocs::Show).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn add(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Json(body): Json<AddCognition>,
) -> Result<(StatusCode, Json<CognitionResponse>), CognitionError> {
    let response = CognitionService::add(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtBookmark>,
    Query(params): Query<ListCognitions>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(key): Path<ResourceKey<CognitionId>>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(
        CognitionService::get(&scope, &GetCognition::builder_v1().key(key).build().into()).await?,
    ))
}
