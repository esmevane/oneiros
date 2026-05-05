use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct ActorRouter;

impl ActorRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/actors",
            ApiRouter::<ServerState>::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| resource_op!(op, ActorDocs::List)).post_with(
                        create,
                        |op| {
                            resource_op!(op, ActorDocs::Create)
                                .response::<201, Json<ActorResponse>>()
                        },
                    ),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| resource_op!(op, ActorDocs::Show)),
                ),
        )
    }
}

async fn create(
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Json(body): Json<CreateActor>,
) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
    let response = ActorService::create(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListActors>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtHost>,
    Path(key): Path<ResourceKey<ActorId>>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(
        ActorService::get(&scope, &GetActor::builder_v1().key(key).build().into()).await?,
    ))
}
