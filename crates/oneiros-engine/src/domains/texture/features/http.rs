use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct TextureRouter;

impl TextureRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/textures",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, TextureDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::put_with(set, |op| {
                        resource_op!(op, TextureDocs::Set)
                            .security_requirement("BearerToken")
                            .response::<200, Json<TextureResponse>>()
                    })
                    .get_with(show, |op| {
                        resource_op!(op, TextureDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, TextureDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<TextureName>,
    Json(mut body): Json<SetTexture>,
) -> Result<(StatusCode, Json<TextureResponse>), TextureError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(TextureService::set(&context, &body).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListTextures>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<TextureName>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(
        TextureService::get(&context, &GetTexture::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<TextureName>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(
        TextureService::remove(&context, &RemoveTexture::builder().name(name).build()).await?,
    ))
}
