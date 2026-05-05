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
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<TextureName>,
    Json(body): Json<SetTexture>,
) -> Result<(StatusCode, Json<TextureResponse>), TextureError> {
    let SetTexture::V1(mut setting) = body;
    setting.name = name;
    let request = SetTexture::V1(setting);
    Ok((
        StatusCode::OK,
        Json(TextureService::set(&scope, &mailbox, &request).await?),
    ))
}

async fn list(
    scope: Scope<AtBookmark>,
    Query(params): Query<ListTextures>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(key): Path<ResourceKey<TextureName>>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(
        TextureService::get(&scope, &GetTexture::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<TextureName>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(
        TextureService::remove(
            &scope,
            &mailbox,
            &RemoveTexture::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
