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
    context: ProjectLog,
    Path(name): Path<NatureName>,
    Json(body): Json<SetNature>,
) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
    let SetNature::V1(mut setting) = body;
    setting.name = name;
    let request = SetNature::V1(setting);
    Ok((
        StatusCode::OK,
        Json(NatureService::set(&context, &request).await?),
    ))
}

async fn list(
    context: ProjectLog,
    Query(params): Query<ListNatures>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(NatureService::list(&context, &params).await?))
}

async fn show(
    context: ProjectLog,
    Path(key): Path<ResourceKey<NatureName>>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::get(&context, &GetNature::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    context: ProjectLog,
    Path(name): Path<NatureName>,
) -> Result<Json<NatureResponse>, NatureError> {
    Ok(Json(
        NatureService::remove(
            &context,
            &RemoveNature::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
