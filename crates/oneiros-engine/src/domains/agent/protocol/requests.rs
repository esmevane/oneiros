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
        summary: "Create an agent",
        description: "Register a new cognitive agent under the current project.",
        content: include_str!("../features/skills/create.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<201, Json<AgentCreatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum CreateAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: AgentName,
            #[builder(into)] pub(crate) persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl CreateAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<CreateAgent>,
    ) -> Result<(StatusCode, Json<AgentResponse>), AgentError> {
        let response = AgentService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Get an agent",
        description: "Look up a specific cognitive agent by name or ID.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<AgentName>>()
                        .response::<200, Json<AgentDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<AgentName>,
        }
    }
}

impl GetAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<AgentName>>,
    ) -> Result<Json<AgentResponse>, AgentError> {
        Ok(Json(
            AgentService::get(&scope, &GetAgent::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List agents",
        description: "List all cognitive agents registered in the current project.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListAgents::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<AgentsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListAgents {
        #[derive(clap::Args)]
        V1 => {
            /// Full-text query against agent name + description. When present,
            /// hits are FTS5-ranked; absent, the listing browses by filters
            /// alone.
            #[arg(long)]
            #[builder(into)]
            pub(crate) query: Option<String>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, query is ignored and the lens
            /// drives selection end-to-end.
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

impl ListAgents {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        scope: Scope<AtBookmark>,
        Query(params): Query<ListAgents>,
    ) -> Result<Json<AgentResponse>, AgentError> {
        let ListAgents::V1(listing) = &params;

        if let Some(source) = listing.lens.as_deref() {
            return Ok(Json(
                AgentLens::new(&scope, state.canons())
                    .list(source, &listing.filters)
                    .await?,
            ));
        }

        Ok(Json(AgentService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Update an agent",
        description: "Modify the configuration or metadata of an existing cognitive agent.",
        content: include_str!("../features/skills/update.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                UpdateAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<AgentName>>()
                        .response::<200, Json<AgentUpdatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum UpdateAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: AgentName,
            #[builder(into)] pub(crate) persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl UpdateAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(_): Path<AgentName>,
        Json(body): Json<UpdateAgent>,
    ) -> Result<Json<AgentResponse>, AgentError> {
        Ok(Json(AgentService::update(&scope, &mailbox, &body).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove an agent",
        description: "Permanently remove a cognitive agent and all associated records from the project.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<AgentName>>()
                        .response::<200, Json<AgentRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: AgentName,
        }
    }
}

impl RemoveAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<AgentName>,
    ) -> Result<Json<AgentResponse>, AgentError> {
        Ok(Json(
            AgentService::remove(
                &scope,
                &mailbox,
                &RemoveAgent::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}

resource_requests! {
    CreateAgent => |this, client| { client.post("/agents", this).await },
    GetAgent => |this, client| {
        let GetAgent::V1(lookup) = this;
        client.get(&format!("/agents/{}", lookup.key)).await
    },
    ListAgents => |this, client| {
        let ListAgents::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

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
        client.get(&format!("/agents?{query}")).await
    },
    UpdateAgent => |this, client| {
        let UpdateAgent::V1(body) = this;
        client
            .put(&format!("/agents/{name}", name = body.name), this)
            .await
    },
    RemoveAgent => |this, client| {
        let RemoveAgent::V1(removal) = this;
        client.delete(&format!("/agents/{}", removal.name)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = AgentRequestType, display = "kebab-case")]
pub(crate) enum AgentRequest {
    CreateAgent(CreateAgent),
    GetAgent(GetAgent),
    ListAgents(ListAgents),
    UpdateAgent(UpdateAgent),
    RemoveAgent(RemoveAgent),
}

resource_root! {
    AgentRequest => {
        label: "agents",
        purpose: "Manage cognitive agents",
        operations: [CreateAgent, GetAgent, ListAgents, UpdateAgent, RemoveAgent],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (AgentRequestType::CreateAgent, "create-agent"),
            (AgentRequestType::GetAgent, "get-agent"),
            (AgentRequestType::ListAgents, "list-agents"),
            (AgentRequestType::UpdateAgent, "update-agent"),
            (AgentRequestType::RemoveAgent, "remove-agent"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }

    #[test]
    fn create_agent_wire_format_is_unwrapped() {
        let request = CreateAgent::V1(CreateAgentV1 {
            name: AgentName::new("test.process"),
            persona: PersonaName::new("process"),
            description: Description::new("desc"),
            prompt: Prompt::new("prompt"),
        });

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["name"], "test.process");
        assert_eq!(json["persona"], "process");
        assert!(
            json.get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
