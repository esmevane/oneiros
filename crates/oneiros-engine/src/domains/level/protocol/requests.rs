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
        summary: "Set a level",
        description: "Define or update a named memory retention tier with its priority and eviction policy.",
        content: include_str!("../features/skills/set.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                SetLevel::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<LevelName>>()
                        .response::<200, Json<LevelSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SetLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: LevelName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl SetLevel {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<LevelName>,
        Json(body): Json<SetLevel>,
    ) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
        let SetLevel::V1(mut setting) = body;
        setting.name = name;
        let request = SetLevel::V1(setting);
        Ok((
            StatusCode::OK,
            Json(LevelService::set(&scope, &mailbox, &request).await?),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Get a level",
        description: "Look up the configuration of a specific memory retention level by name.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetLevel::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<LevelName>>()
                        .response::<200, Json<LevelDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<LevelName>,
        }
    }
}

impl GetLevel {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<LevelName>>,
    ) -> Result<Json<LevelResponse>, LevelError> {
        Ok(Json(
            LevelService::get(&scope, &GetLevel::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List levels",
        description: "List all memory retention levels defined for the current project.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListLevels::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<LevelsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListLevels {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListLevels {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListLevels>,
    ) -> Result<Json<LevelResponse>, LevelError> {
        Ok(Json(LevelService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove a level",
        description: "Delete a memory retention level, preventing new memories from being classified under it.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveLevel::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<LevelName>>()
                        .response::<200, Json<LevelRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: LevelName,
        }
    }
}

impl RemoveLevel {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<LevelName>,
    ) -> Result<Json<LevelResponse>, LevelError> {
        Ok(Json(
            LevelService::remove(
                &scope,
                &mailbox,
                &RemoveLevel::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}

resource_requests! {
    SetLevel => |this, client| {
        let SetLevel::V1(body) = this;
        client.put(&format!("/levels/{}", body.name), this).await
    },
    GetLevel => |this, client| {
        let GetLevel::V1(lookup) = this;
        client.get(&format!("/levels/{}", lookup.key)).await
    },
    RemoveLevel => |this, client| {
        let RemoveLevel::V1(removal) = this;
        client.delete(&format!("/levels/{}", removal.name)).await
    },
    ListLevels => |this, client| {
        let ListLevels::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/levels?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = LevelRequestType, display = "kebab-case")]
pub(crate) enum LevelRequest {
    SetLevel(SetLevel),
    GetLevel(GetLevel),
    ListLevels(ListLevels),
    RemoveLevel(RemoveLevel),
}

resource_root! {
    LevelRequest => {
        label: "levels",
        purpose: "Define memory retention tiers",
        operations: [SetLevel, GetLevel, ListLevels, RemoveLevel],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (LevelRequestType::SetLevel, "set-level"),
            (LevelRequestType::GetLevel, "get-level"),
            (LevelRequestType::ListLevels, "list-levels"),
            (LevelRequestType::RemoveLevel, "remove-level"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
