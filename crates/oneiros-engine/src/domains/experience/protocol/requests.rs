use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Create an experience",
        description: "Mark a meaningful moment in the agent's timeline with a description and sensation.",
        content: include_str!("../features/skills/create.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateExperience::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<201, Json<ExperienceCreatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum CreateExperience {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) sensation: SensationName,
            #[builder(into)] pub(crate) description: Description,
        }
    }
}

impl CreateExperience {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<CreateExperience>,
    ) -> Result<(StatusCode, Json<ExperienceResponse>), ExperienceError> {
        let response = ExperienceService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Get an experience",
        description: "Retrieve the full record of a specific marked moment by ID.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetExperience::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<IdPathParam<ExperienceId>>()
                        .response::<200, Json<ExperienceDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetExperience {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ExperienceId>,
        }
    }
}

impl GetExperience {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<ExperienceId>>,
    ) -> Result<Json<ExperienceResponse>, ExperienceError> {
        Ok(Json(
            ExperienceService::get(&scope, &GetExperience::builder_v1().key(key).build().into())
                .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List experiences",
        description: "List all marked experiences in the agent's history, optionally filtered by sensation.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListExperiences::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<ExperiencesResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListExperiences {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) agent: Option<AgentName>,
            #[arg(long)]
            pub(crate) sensation: Option<SensationName>,
            /// Full-text query against experience description. When present,
            /// hits are FTS5-ranked; absent, the listing browses by filters
            /// alone.
            #[arg(long)]
            #[builder(into)]
            pub(crate) query: Option<String>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, agent/sensation/query are ignored
            /// and the lens drives selection end-to-end.
            #[arg(long)]
            #[builder(into)]
            pub(crate) lens: Option<String>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListExperiences {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        scope: Scope<AtBookmark>,
        Query(params): Query<ListExperiences>,
    ) -> Result<Json<ExperienceResponse>, ExperienceError> {
        let ListExperiences::V1(listing) = &params;
        if let Some(source) = listing.lens.as_deref() {
            return Ok(Json(
                ExperienceLens::new(&scope, state.canons())
                    .list(source, &listing.filters)
                    .await?,
            ));
        }
        Ok(Json(ExperienceService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}/description",
        summary: "Update an experience description",
        description: "Modify the description of a specific marked moment.",
        content: include_str!("../features/skills/update.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                UpdateExperienceDescription::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<IdPathParam<ExperienceId>>()
                        .response::<200, Json<ExperienceUpdatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum UpdateExperienceDescription {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: ExperienceId,
            #[builder(into)] pub(crate) description: Description,
        }
    }
}

impl UpdateExperienceDescription {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(id): Path<ExperienceId>,
        Json(body): Json<UpdateExperienceDescription>,
    ) -> Result<Json<ExperienceResponse>, ExperienceError> {
        let UpdateExperienceDescription::V1(inner) = &body;
        Ok(Json(
            ExperienceService::update_description(
                &scope,
                &mailbox,
                &UpdateExperienceDescription::builder_v1()
                    .id(id)
                    .description(inner.description.clone())
                    .build()
                    .into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}/sensation",
        summary: "Update an experience sensation",
        description: "Modify the sensation of a specific marked moment.",
        content: include_str!("../features/skills/update.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                UpdateExperienceSensation::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<IdPathParam<ExperienceId>>()
                        .response::<200, Json<ExperienceUpdatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum UpdateExperienceSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: ExperienceId,
            #[builder(into)] pub(crate) sensation: SensationName,
        }
    }
}

impl UpdateExperienceSensation {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(id): Path<ExperienceId>,
        Json(body): Json<UpdateExperienceSensation>,
    ) -> Result<Json<ExperienceResponse>, ExperienceError> {
        let UpdateExperienceSensation::V1(inner) = &body;
        Ok(Json(
            ExperienceService::update_sensation(
                &scope,
                &mailbox,
                &UpdateExperienceSensation::builder_v1()
                    .id(id)
                    .sensation(inner.sensation.clone())
                    .build()
                    .into(),
            )
            .await?,
        ))
    }
}

resource_requests! {
    CreateExperience => |this, client| {
        client.post("/experiences", this).await
    },
    GetExperience => |this, client| {
        let GetExperience::V1(lookup) = this;
        client.get(&format!("/experiences/{}", lookup.key)).await
    },
    ListExperiences => |this, client| {
        let ListExperiences::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(sensation_name) = &listing.sensation {
            params.push(("sensation", sensation_name.to_string()));
        }

        if let Some(query) = &listing.query {
            params.push(("query", query.clone()));
        }

        if let Some(lens) = &listing.lens {
            params.push(("lens", lens.clone()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        client.get(&format!("/experiences?{query}")).await
    },
    UpdateExperienceDescription => |this, client| {
        let UpdateExperienceDescription::V1(update) = this;
        client
            .put(&format!("/experiences/{}/description", update.id), this)
            .await
    },
    UpdateExperienceSensation => |this, client| {
        let UpdateExperienceSensation::V1(update) = this;
        client
            .put(&format!("/experiences/{}/sensation", update.id), this)
            .await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ExperienceRequestType, display = "kebab-case")]
pub(crate) enum ExperienceRequest {
    CreateExperience(CreateExperience),
    GetExperience(GetExperience),
    ListExperiences(ListExperiences),
    UpdateExperienceDescription(UpdateExperienceDescription),
    UpdateExperienceSensation(UpdateExperienceSensation),
}

resource_root! {
    ExperienceRequest => {
        label: "experiences",
        purpose: "Mark and revisit meaningful moments",
        operations: [CreateExperience, GetExperience, ListExperiences, UpdateExperienceDescription, UpdateExperienceSensation],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ExperienceRequestType::CreateExperience, "create-experience"),
            (ExperienceRequestType::GetExperience, "get-experience"),
            (ExperienceRequestType::ListExperiences, "list-experiences"),
            (
                ExperienceRequestType::UpdateExperienceDescription,
                "update-experience-description",
            ),
            (
                ExperienceRequestType::UpdateExperienceSensation,
                "update-experience-sensation",
            ),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
