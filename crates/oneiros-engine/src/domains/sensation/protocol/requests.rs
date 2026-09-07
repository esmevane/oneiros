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
        summary: "Define a sensation",
        description: "Create or update a quality of experience in the project's vocabulary.",
        content: include_str!("../features/skills/set.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                SetSensation::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<SensationName>>()
                        .response::<200, Json<SensationSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SetSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: SensationName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl SetSensation {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<SensationName>,
        Json(body): Json<SetSensation>,
    ) -> Result<(StatusCode, Json<SensationResponse>), SensationError> {
        let SetSensation::V1(mut setting) = body;
        setting.name = name;
        let request = SetSensation::V1(setting);
        Ok((
            StatusCode::OK,
            Json(SensationService::set(&scope, &mailbox, &request).await?),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Show a sensation",
        description: "Retrieve a single quality of experience by name.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetSensation::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<SensationName>>()
                        .response::<200, Json<SensationDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<SensationName>,
        }
    }
}

impl GetSensation {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<SensationName>>,
    ) -> Result<Json<SensationResponse>, SensationError> {
        Ok(Json(
            SensationService::get(&scope, &GetSensation::builder_v1().key(key).build().into())
                .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove a sensation",
        description: "Delete a quality of experience from the project's vocabulary.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveSensation::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<SensationName>>()
                        .response::<200, Json<SensationRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: SensationName,
        }
    }
}

impl RemoveSensation {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<SensationName>,
    ) -> Result<Json<SensationResponse>, SensationError> {
        Ok(Json(
            SensationService::remove(
                &scope,
                &mailbox,
                &RemoveSensation::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List sensations",
        description: "See all defined qualities of experience available to agents.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListSensations::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<SensationsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListSensations {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListSensations {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListSensations>,
    ) -> Result<Json<SensationResponse>, SensationError> {
        Ok(Json(SensationService::list(&scope, &params).await?))
    }
}

resource_requests! {
    SetSensation => |this, client| {
        let SetSensation::V1(body) = this;
        client.put(&format!("/sensations/{}", body.name), this).await
    },
    GetSensation => |this, client| {
        let GetSensation::V1(lookup) = this;
        client.get(&format!("/sensations/{}", lookup.key)).await
    },
    RemoveSensation => |this, client| {
        let RemoveSensation::V1(removal) = this;
        client.delete(&format!("/sensations/{}", removal.name)).await
    },
    ListSensations => |this, client| {
        let ListSensations::V1(listing) = this;
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset,);
        client.get(&format!("/sensations?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SensationRequestType, display = "kebab-case")]
pub(crate) enum SensationRequest {
    SetSensation(SetSensation),
    GetSensation(GetSensation),
    ListSensations(ListSensations),
    RemoveSensation(RemoveSensation),
}

resource_root! {
    SensationRequest => {
        label: "sensations",
        purpose: "Define qualities of experience",
        operations: [SetSensation, GetSensation, ListSensations, RemoveSensation],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (SensationRequestType::SetSensation, "set-sensation"),
            (SensationRequestType::GetSensation, "get-sensation"),
            (SensationRequestType::ListSensations, "list-sensations"),
            (SensationRequestType::RemoveSensation, "remove-sensation"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
