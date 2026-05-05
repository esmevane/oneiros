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
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<UrgeName>,
    Json(body): Json<SetUrge>,
) -> Result<(StatusCode, Json<UrgeResponse>), UrgeError> {
    let SetUrge::V1(mut setting) = body;
    setting.name = name;
    let request = SetUrge::V1(setting);
    Ok((
        StatusCode::OK,
        Json(UrgeService::set(&scope, &mailbox, &request).await?),
    ))
}

async fn list(
    scope: Scope<AtBookmark>,
    Query(params): Query<ListUrges>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(UrgeService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(key): Path<ResourceKey<UrgeName>>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(
        UrgeService::get(&scope, &GetUrge::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<UrgeName>,
) -> Result<Json<UrgeResponse>, UrgeError> {
    Ok(Json(
        UrgeService::remove(
            &scope,
            &mailbox,
            &RemoveUrge::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
