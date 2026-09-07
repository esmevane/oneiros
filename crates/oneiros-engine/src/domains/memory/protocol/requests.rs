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
        summary: "Add a memory",
        description: "Store a piece of consolidated knowledge for the agent at a specified retention level.",
        content: include_str!("../features/skills/add.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                AddMemory::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<201, Json<MemoryAddedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum AddMemory {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) level: LevelName,
            #[builder(into)] pub(crate) content: Content,
        }
    }
}

impl AddMemory {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<AddMemory>,
    ) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
        let response = MemoryService::add(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Get a memory",
        description: "Retrieve the full content of a specific memory by ID.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetMemory::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<IdPathParam<MemoryId>>()
                        .response::<200, Json<MemoryDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetMemory {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<MemoryId>,
        }
    }
}

impl GetMemory {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<MemoryId>>,
    ) -> Result<Json<MemoryResponse>, MemoryError> {
        Ok(Json(
            MemoryService::get(&scope, &GetMemory::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List memories",
        description: "List all memories held by the agent, optionally filtered by retention level.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListMemories::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<MemoriesResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListMemories {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) agent: Option<AgentName>,
            #[arg(long)]
            pub(crate) level: Option<LevelName>,
            /// Full-text query against memory content. When present, hits
            /// are FTS5-ranked; absent, the listing browses by filters alone.
            #[arg(long)]
            #[builder(into)]
            pub(crate) query: Option<String>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, agent/level/query are ignored
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

impl ListMemories {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        scope: Scope<AtBookmark>,
        Query(params): Query<ListMemories>,
    ) -> Result<Json<MemoryResponse>, MemoryError> {
        let ListMemories::V1(listing) = &params;
        if let Some(source) = listing.lens.as_deref() {
            return Ok(Json(
                MemoryLens::new(&scope, state.canons())
                    .list(source, &listing.filters)
                    .await?,
            ));
        }
        Ok(Json(MemoryService::list(&scope, &params).await?))
    }
}

resource_requests! {
    AddMemory => |this, client| {
        client.post("/memories", this).await
    },
    GetMemory => |this, client| {
        let GetMemory::V1(lookup) = this;
        client.get(&format!("/memories/{}", lookup.key)).await
    },
    ListMemories => |this, client| {
        let ListMemories::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(level_name) = &listing.level {
            params.push(("level", level_name.to_string()));
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

        client.get(&format!("/memories?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = MemoryRequestType, display = "kebab-case")]
pub(crate) enum MemoryRequest {
    AddMemory(AddMemory),
    GetMemory(GetMemory),
    ListMemories(ListMemories),
}

resource_root! {
    MemoryRequest => {
        label: "memories",
        purpose: "Consolidate and review knowledge",
        operations: [AddMemory, GetMemory, ListMemories],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (MemoryRequestType::AddMemory, "add-memory"),
            (MemoryRequestType::GetMemory, "get-memory"),
            (MemoryRequestType::ListMemories, "list-memories"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
