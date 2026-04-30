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
    context: HostLog,
    Json(body): Json<AddPeer>,
) -> Result<(StatusCode, Json<PeerResponse>), PeerError> {
    let response = PeerService::add(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: HostLog,
    Query(params): Query<ListPeers>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(PeerService::list(&context, &params).await?))
}

async fn show(
    context: HostLog,
    Path(key): Path<ResourceKey<PeerId>>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(
        PeerService::get(&context, &GetPeer::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(context: HostLog, Path(id): Path<PeerId>) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(
        PeerService::remove(&context, &RemovePeer::builder_v1().id(id).build().into()).await?,
    ))
}
