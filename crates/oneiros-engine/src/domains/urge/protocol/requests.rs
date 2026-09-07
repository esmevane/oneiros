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
        summary: "Define a urge",
        description: "Create or update a quality of cognitive drive in the project's vocabulary.",
        content: include_str!("../features/skills/set.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                SetUrge::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<UrgeName>>()
                        .response::<200, Json<UrgeSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SetUrge {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: UrgeName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl SetUrge {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<UrgeName>,
        Json(body): Json<SetUrge>,
    ) -> Result<(StatusCode, Json<UrgeResponse>), UrgeError> {
        let SetUrge::V1(mut setting) = body;
        setting.name = name;
        let request = SetUrge::V1(setting);
        Ok((
            StatusCode::OK,
            Json(UrgeService::set(&scope, &mailbox, &request).await?),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Show a urge",
        description: "Retrieve a single quality of cognitive drive by name.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetUrge::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<UrgeName>>()
                        .response::<200, Json<UrgeDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetUrge {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<UrgeName>,
        }
    }
}

impl GetUrge {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<UrgeName>>,
    ) -> Result<Json<UrgeResponse>, UrgeError> {
        Ok(Json(
            UrgeService::get(&scope, &GetUrge::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove a urge",
        description: "Delete a quality of cognitive drive from the project's vocabulary.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveUrge::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<UrgeName>>()
                        .response::<200, Json<UrgeRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveUrge {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: UrgeName,
        }
    }
}

impl RemoveUrge {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<UrgeName>,
    ) -> Result<Json<UrgeResponse>, UrgeError> {
        Ok(Json(
            UrgeService::remove(
                &scope,
                &mailbox,
                &RemoveUrge::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List urges",
        description: "See all defined qualities of cognitive drive available to agents.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListUrges::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<UrgesResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListUrges {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListUrges {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListUrges>,
    ) -> Result<Json<UrgeResponse>, UrgeError> {
        Ok(Json(UrgeService::list(&scope, &params).await?))
    }
}

resource_requests! {
    SetUrge => |this, client| {
        let SetUrge::V1(body) = this;
        client.put(&format!("/urges/{}", body.name), this).await
    },
    GetUrge => |this, client| {
        let GetUrge::V1(lookup) = this;
        client.get(&format!("/urges/{}", lookup.key)).await
    },
    RemoveUrge => |this, client| {
        let RemoveUrge::V1(removal) = this;
        client.delete(&format!("/urges/{}", removal.name)).await
    },
    ListUrges => |this, client| {
        let ListUrges::V1(listing) = this;
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset,);
        client.get(&format!("/urges?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = UrgeRequestType, display = "kebab-case")]
pub(crate) enum UrgeRequest {
    SetUrge(SetUrge),
    GetUrge(GetUrge),
    ListUrges(ListUrges),
    RemoveUrge(RemoveUrge),
}

resource_root! {
    UrgeRequest => {
        label: "urges",
        purpose: "Define qualities of cognitive drive",
        operations: [SetUrge, GetUrge, ListUrges, RemoveUrge],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (UrgeRequestType::SetUrge, "set-urge"),
            (UrgeRequestType::GetUrge, "get-urge"),
            (UrgeRequestType::ListUrges, "list-urges"),
            (UrgeRequestType::RemoveUrge, "remove-urge"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
