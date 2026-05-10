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
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
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
