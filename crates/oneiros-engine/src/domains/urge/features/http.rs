use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct UrgeRouter;

impl UrgeRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/urges",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, UrgeDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::put_with(set, |op| {
                        resource_op!(op, UrgeDocs::Set)
                            .security_requirement("BearerToken")
                            .response::<200, Json<UrgeResponse>>()
                    })
                    .get_with(show, |op| {
                        resource_op!(op, UrgeDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, UrgeDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<UrgeName>,
    Json(mut body): Json<SetUrge>,
) -> Result<(StatusCode, Json<UrgeResponse>), UrgeError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(UrgeService::set(&context, &body).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListUrges>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<UrgeName>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(
        UrgeService::get(&context, &GetUrge::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<UrgeName>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(
        UrgeService::remove(&context, &RemoveUrge::builder().name(name).build()).await?,
    ))
}
