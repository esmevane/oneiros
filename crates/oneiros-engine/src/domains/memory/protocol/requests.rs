use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct AddMemory {
    #[builder(into)]
    pub(crate) agent: AgentName,
    #[builder(into)]
    pub(crate) level: LevelName,
    #[builder(into)]
    pub(crate) content: Content,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetMemory {
    #[builder(into)]
    pub(crate) id: MemoryId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListMemories {
    #[arg(long)]
    pub(crate) agent: Option<AgentName>,
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
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
