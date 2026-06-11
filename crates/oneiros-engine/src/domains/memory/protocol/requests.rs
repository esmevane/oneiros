use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AddMemory {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) level: LevelName,
            #[builder(into)] pub(crate) content: Content,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetMemory {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<MemoryId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
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
