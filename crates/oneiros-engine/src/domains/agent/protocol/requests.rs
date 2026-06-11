use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
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

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<AgentName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
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

versioned! {
    #[derive(JsonSchema)]
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

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: AgentName,
        }
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
