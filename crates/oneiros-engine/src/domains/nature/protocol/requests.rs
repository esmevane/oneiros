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
        summary: "Set a nature",
        description: "Define or update a named relationship kind that can be used to type connections between entities.",
        content: include_str!("../features/skills/set.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                SetNature::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<NatureName>>()
                        .response::<200, Json<NatureSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SetNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: NatureName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl SetNature {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<NatureName>,
        Json(body): Json<SetNature>,
    ) -> Result<(StatusCode, Json<NatureResponse>), NatureError> {
        let SetNature::V1(mut setting) = body;
        setting.name = name;
        let request = SetNature::V1(setting);
        Ok((
            StatusCode::OK,
            Json(NatureService::set(&scope, &mailbox, &request).await?),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Get a nature",
        description: "Look up the definition of a specific relationship kind by name.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetNature::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<NatureName>>()
                        .response::<200, Json<NatureDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<NatureName>,
        }
    }
}

impl GetNature {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<NatureName>>,
    ) -> Result<Json<NatureResponse>, NatureError> {
        Ok(Json(
            NatureService::get(&scope, &GetNature::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List natures",
        description: "List all relationship kinds defined for the current project.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListNatures::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<NaturesResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListNatures {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListNatures {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListNatures>,
    ) -> Result<Json<NatureResponse>, NatureError> {
        Ok(Json(NatureService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove a nature",
        description: "Delete a relationship kind, preventing it from being assigned to new connections.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveNature::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<NatureName>>()
                        .response::<200, Json<NatureRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: NatureName,
        }
    }
}

impl RemoveNature {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<NatureName>,
    ) -> Result<Json<NatureResponse>, NatureError> {
        Ok(Json(
            NatureService::remove(
                &scope,
                &mailbox,
                &RemoveNature::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}

resource_requests! {
    SetNature => |this, client| {
        let SetNature::V1(body) = this;
        client.put(&format!("/natures/{}", body.name), this).await
    },
    GetNature => |this, client| {
        let GetNature::V1(lookup) = this;
        client.get(&format!("/natures/{}", lookup.key)).await
    },
    RemoveNature => |this, client| {
        let RemoveNature::V1(removal) = this;
        client.delete(&format!("/natures/{}", removal.name)).await
    },
    ListNatures => |this, client| {
        let ListNatures::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/natures?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = NatureRequestType, display = "kebab-case")]
pub(crate) enum NatureRequest {
    SetNature(SetNature),
    GetNature(GetNature),
    ListNatures(ListNatures),
    RemoveNature(RemoveNature),
}

resource_root! {
    NatureRequest => {
        label: "natures",
        purpose: "Define kinds of relationships",
        operations: [SetNature, GetNature, ListNatures, RemoveNature],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (NatureRequestType::SetNature, "set-nature"),
            (NatureRequestType::GetNature, "get-nature"),
            (NatureRequestType::ListNatures, "list-natures"),
            (NatureRequestType::RemoveNature, "remove-nature"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
