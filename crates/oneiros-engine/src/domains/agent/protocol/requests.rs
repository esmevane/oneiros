use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateAgent {
    #[builder(into)]
    pub name: AgentName,
    #[builder(into)]
    pub persona: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetAgent {
    #[builder(into)]
    pub name: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveAgent {
    #[builder(into)]
    pub name: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UpdateAgent {
    #[builder(into)]
    pub name: AgentName,
    #[builder(into)]
    pub persona: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(into)]
    pub description: Description,
    #[arg(long, default_value = "")]
    #[builder(into)]
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = AgentRequestType, display = "kebab-case")]
pub enum AgentRequest {
    CreateAgent(CreateAgent),
    GetAgent(GetAgent),
    ListAgents,
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
