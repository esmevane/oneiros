use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct PeerRouter;

impl PeerRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/peers",
            ApiRouter::<ServerState>::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| resource_op!(op, PeerDocs::List))
                        .post_with(add, |op| {
                            resource_op!(op, PeerDocs::Add).response::<201, Json<PeerResponse>>()
                        }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| resource_op!(op, PeerDocs::Show))
                        .delete_with(remove, |op| resource_op!(op, PeerDocs::Remove)),
                ),
        )
    }
}

async fn add(
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Json(body): Json<AddPeer>,
) -> Result<(StatusCode, Json<PeerResponse>), PeerError> {
    let response = PeerService::add(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListPeers>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(PeerService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtHost>,
    Path(key): Path<ResourceKey<PeerId>>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(
        PeerService::get(&scope, &GetPeer::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Path(id): Path<PeerId>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(
        PeerService::remove(
            &scope,
            &mailbox,
            &RemovePeer::builder_v1().id(id).build().into(),
        )
        .await?,
    ))
}
