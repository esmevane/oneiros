use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct LevelRouter;

impl LevelRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/levels",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, LevelDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::put_with(set, |op| {
                        resource_op!(op, LevelDocs::Set)
                            .security_requirement("BearerToken")
                            .response::<200, Json<LevelResponse>>()
                    })
                    .get_with(show, |op| {
                        resource_op!(op, LevelDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, LevelDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn set(
    context: ProjectLog,
    Path(name): Path<LevelName>,
    Json(body): Json<SetLevel>,
) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
    let SetLevel::V1(mut setting) = body;
    setting.name = name;
    let request = SetLevel::V1(setting);
    Ok((
        StatusCode::OK,
        Json(LevelService::set(&context, &request).await?),
    ))
}

async fn list(
    context: ProjectLog,
    Query(params): Query<ListLevels>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::list(&context, &params).await?))
}

async fn show(
    context: ProjectLog,
    Path(key): Path<ResourceKey<LevelName>>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(
        LevelService::get(&context, &GetLevel::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    context: ProjectLog,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(
        LevelService::remove(
            &context,
            &RemoveLevel::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
