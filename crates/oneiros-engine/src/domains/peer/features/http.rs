use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub(crate) struct PeerRouter;

impl PeerRouter {
    pub(crate) fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/peers",
            Router::<ServerState>::new()
                .route("/", routing::get(list).post(add))
                .route("/{id}", routing::get(show).delete(remove)),
        )
    }
}

async fn add(
    context: SystemContext,
    Json(body): Json<AddPeer>,
) -> Result<(StatusCode, Json<PeerResponse>), PeerError> {
    let response = PeerService::add(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: SystemContext,
    Query(params): Query<ListPeers>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(PeerService::list(&context, &params).await?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<PeerId>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(
        PeerService::get(&context, &GetPeer::builder().id(id).build()).await?,
    ))
}

async fn remove(
    context: SystemContext,
    Path(id): Path<PeerId>,
) -> Result<Json<PeerResponse>, PeerError> {
    Ok(Json(
        PeerService::remove(&context, &RemovePeer::builder().id(id).build()).await?,
    ))
}
