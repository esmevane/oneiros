use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct ConnectionRouter;

impl ConnectionRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/connections",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, ConnectionDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(create, |op| {
                        resource_op!(op, ConnectionDocs::Create)
                            .security_requirement("BearerToken")
                            .response::<201, Json<ConnectionResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, ConnectionDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, ConnectionDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateConnection>,
) -> Result<(StatusCode, Json<ConnectionResponse>), ConnectionError> {
    let response = ConnectionService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListConnections>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(ConnectionService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(key): Path<ResourceKey<ConnectionId>>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(
        ConnectionService::get(&context, &GetConnection::builder().key(key).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(id): Path<ConnectionId>,
) -> Result<Json<ConnectionResponse>, ConnectionError> {
    Ok(Json(
        ConnectionService::remove(&context, &RemoveConnection::builder().id(id).build()).await?,
    ))
}
