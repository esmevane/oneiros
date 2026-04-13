use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct CreateAgent {
    #[builder(into)]
    pub(crate) name: AgentName,
    #[builder(into)]
    pub(crate) persona: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetAgent {
    #[builder(into)]
    pub(crate) name: AgentName,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListAgents {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RemoveAgent {
    #[builder(into)]
    pub(crate) name: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct UpdateAgent {
    #[builder(into)]
    pub(crate) name: AgentName,
    #[builder(into)]
    pub(crate) persona: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(into)]
    pub(crate) description: Description,
    #[arg(long, default_value = "")]
    #[builder(into)]
    pub(crate) prompt: Prompt,
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
}
