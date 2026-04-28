use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct SensationRouter;

impl SensationRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/sensations",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, SensationDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::put_with(set, |op| {
                        resource_op!(op, SensationDocs::Set)
                            .security_requirement("BearerToken")
                            .response::<200, Json<SensationResponse>>()
                    })
                    .get_with(show, |op| {
                        resource_op!(op, SensationDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, SensationDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<SensationName>,
    Json(body): Json<SetSensation>,
) -> Result<(StatusCode, Json<SensationResponse>), SensationError> {
    let SetSensation::V1(mut setting) = body;
    setting.name = name;
    let request = SetSensation::V1(setting);
    Ok((
        StatusCode::OK,
        Json(SensationService::set(&context, &request).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListSensations>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(key): Path<ResourceKey<SensationName>>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(
        SensationService::get(
            &context,
            &GetSensation::builder_v1().key(key).build().into(),
        )
        .await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<SensationName>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(
        SensationService::remove(
            &context,
            &RemoveSensation::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
