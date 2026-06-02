use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::Path, extract::Query};

use crate::*;

pub(crate) struct FollowRouter;

impl FollowRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/follows",
            ApiRouter::<ServerState>::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, FollowDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, FollowDocs::Get).security_requirement("BearerToken").input::<IdPathParam<FollowId>>()
                    }),
                ),
        )
    }
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListFollows>,
) -> Result<Json<Response<FollowResponse>>, FollowError> {
    let ListFollows::V1(listing) = &params;
    let listed = FollowService::list(&scope, &listing.filters).await?;

    let items: Vec<Response<FollowFoundResponse>> = listed
        .items
        .into_iter()
        .map(|follow| {
            Response::new(
                FollowFoundResponse::builder_v1()
                    .follow(follow)
                    .build()
                    .into(),
            )
        })
        .collect();

    let listed = Listed::new(items, listed.total);
    let inner = FollowsResponse::builder_v1().follows(listed).build().into();

    Ok(Json(Response::new(FollowResponse::Listed(inner))))
}

async fn show(
    scope: Scope<AtHost>,
    Path(key): Path<ResourceKey<FollowId>>,
) -> Result<Json<Response<FollowResponse>>, FollowError> {
    let id = key.resolve()?;
    let follow = FollowService::get(&scope, id).await?;

    let inner = FollowFoundResponse::builder_v1()
        .follow(follow)
        .build()
        .into();

    Ok(Json(Response::new(FollowResponse::Found(inner))))
}
