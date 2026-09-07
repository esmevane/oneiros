use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Define a texture",
        description: "Create or update a quality of thought in the project's vocabulary.",
        content: include_str!("../features/skills/set.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                SetTexture::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<TextureName>>()
                        .response::<200, Json<TextureSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SetTexture {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: TextureName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl SetTexture {
    pub(crate) async fn handler(
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
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Show a texture",
        description: "Retrieve a single quality of thought by name.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetTexture::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<TextureName>>()
                        .response::<200, Json<TextureDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetTexture {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TextureName>,
        }
    }
}

impl GetTexture {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<TextureName>>,
    ) -> Result<Json<TextureResponse>, TextureError> {
        Ok(Json(
            TextureService::get(&scope, &GetTexture::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove a texture",
        description: "Delete a quality of thought from the project's vocabulary.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveTexture::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<TextureName>>()
                        .response::<200, Json<TextureRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveTexture {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: TextureName,
        }
    }
}

impl RemoveTexture {
    pub(crate) async fn handler(
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
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List textures",
        description: "See all defined qualities of thought available to agents.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListTextures::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<TexturesResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListTextures {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListTextures {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListTextures>,
    ) -> Result<Json<TextureResponse>, TextureError> {
        Ok(Json(TextureService::list(&scope, &params).await?))
    }
}

resource_requests! {
    SetTexture => |this, client| {
        let SetTexture::V1(body) = this;
        client.put(&format!("/textures/{}", body.name), this).await
    },
    GetTexture => |this, client| {
        let GetTexture::V1(lookup) = this;
        client.get(&format!("/textures/{}", lookup.key)).await
    },
    RemoveTexture => |this, client| {
        let RemoveTexture::V1(removal) = this;
        client.delete(&format!("/textures/{}", removal.name)).await
    },
    ListTextures => |this, client| {
        let ListTextures::V1(listing) = this;
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset,);
        client.get(&format!("/textures?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TextureRequestType, display = "kebab-case")]
pub(crate) enum TextureRequest {
    SetTexture(SetTexture),
    GetTexture(GetTexture),
    ListTextures(ListTextures),
    RemoveTexture(RemoveTexture),
}

resource_root! {
    TextureRequest => {
        label: "textures",
        purpose: "Define qualities of thought",
        operations: [SetTexture, GetTexture, ListTextures, RemoveTexture],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (TextureRequestType::SetTexture, "set-texture"),
            (TextureRequestType::GetTexture, "get-texture"),
            (TextureRequestType::ListTextures, "list-textures"),
            (TextureRequestType::RemoveTexture, "remove-texture"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
