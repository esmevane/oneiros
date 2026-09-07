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
        summary: "Add a thought",
        description: "Record a new thought for the agent, tagged with a texture that describes its nature.",
        content: include_str!("../features/skills/add.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                AddCognition::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<201, Json<CognitionAddedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum AddCognition {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) texture: TextureName,
            #[builder(into)] pub(crate) content: Content,
        }
    }
}

impl AddCognition {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<AddCognition>,
    ) -> Result<(StatusCode, Json<CognitionResponse>), CognitionError> {
        let response = CognitionService::add(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Get a thought",
        description: "Retrieve the full content of a specific thought by ID.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetCognition::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<IdPathParam<CognitionId>>()
                        .response::<200, Json<CognitionDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetCognition {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<CognitionId>,
        }
    }
}

impl GetCognition {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<CognitionId>>,
    ) -> Result<Json<CognitionResponse>, CognitionError> {
        Ok(Json(
            CognitionService::get(&scope, &GetCognition::builder_v1().key(key).build().into())
                .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List thoughts",
        description: "List all thoughts recorded by the agent, optionally filtered by texture.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListCognitions::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<CognitionsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListCognitions {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) agent: Option<AgentName>,
            #[arg(long)]
            pub(crate) texture: Option<TextureName>,
            /// Full-text query against cognition content. When present, hits
            /// are FTS5-ranked; absent, the listing browses by filters alone.
            #[arg(long)]
            #[builder(into)]
            pub(crate) query: Option<String>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, agent/texture/query are ignored
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

impl ListCognitions {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        scope: Scope<AtBookmark>,
        Query(params): Query<ListCognitions>,
    ) -> Result<Json<CognitionResponse>, CognitionError> {
        let ListCognitions::V1(listing) = &params;
        if let Some(source) = listing.lens.as_deref() {
            return Ok(Json(
                CognitionLens::new(&scope, state.canons())
                    .list(source, &listing.filters)
                    .await?,
            ));
        }
        Ok(Json(CognitionService::list(&scope, &params).await?))
    }
}

resource_requests! {
    AddCognition => |this, client| { client.post("/cognitions", this).await },
    GetCognition => |this, client| {
        let GetCognition::V1(lookup) = this;
        client.get(&format!("/cognitions/{}", lookup.key)).await
    },
    ListCognitions => |this, client| {
        let ListCognitions::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(texture_name) = &listing.texture {
            params.push(("texture", texture_name.to_string()));
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

        client.get(&format!("/cognitions?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = CognitionRequestType, display = "kebab-case")]
pub(crate) enum CognitionRequest {
    AddCognition(AddCognition),
    GetCognition(GetCognition),
    ListCognitions(ListCognitions),
}

resource_root! {
    CognitionRequest => {
        label: "cognitions",
        purpose: "Record and review thoughts",
        operations: [AddCognition, GetCognition, ListCognitions],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (CognitionRequestType::AddCognition, "add-cognition"),
            (CognitionRequestType::GetCognition, "get-cognition"),
            (CognitionRequestType::ListCognitions, "list-cognitions"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
