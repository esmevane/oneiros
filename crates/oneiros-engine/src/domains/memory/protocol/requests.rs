use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum AddMemory {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
            #[builder(into)] pub level: LevelName,
            #[builder(into)] pub content: Content,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetMemory {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<MemoryId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListMemories {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub agent: Option<AgentName>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = MemoryRequestType, display = "kebab-case")]
pub enum MemoryRequest {
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
