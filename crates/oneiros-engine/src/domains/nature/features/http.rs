use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct NatureRouter;

impl NatureRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/natures",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, NatureDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::put_with(set, |op| {
                        resource_op!(op, NatureDocs::Set)
                            .security_requirement("BearerToken")
                            .response::<200, Json<NatureResponse>>()
                    })
                    .get_with(show, |op| {
                        resource_op!(op, NatureDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, NatureDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<NatureName>,
    Json(mut body): Json<SetNature>,
) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(NatureService::set(&context, &body).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListNatures>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(key): Path<ResourceKey<NatureName>>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::get(&context, &GetNature::builder().key(key).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::remove(&context, &RemoveNature::builder().name(name).build()).await?,
    ))
}
