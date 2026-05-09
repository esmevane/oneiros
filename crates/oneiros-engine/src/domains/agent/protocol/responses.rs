use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = AgentResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum AgentResponse {
    AgentCreated(AgentCreatedResponse),
    AgentDetails(AgentDetailsResponse),
    Agents(AgentsResponse),
    NoAgents,
    AgentUpdated(AgentUpdatedResponse),
    AgentRemoved(AgentRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AgentCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) agent: Agent }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AgentDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) agent: Agent }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AgentsResponse {
        V1 => { pub(crate) items: Vec<Agent>, pub(crate) total: usize }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AgentUpdatedResponse {
        V1 => { #[serde(flatten)] pub(crate) agent: Agent }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AgentRemovedResponse {
        V1 => { #[builder(into)] pub(crate) name: AgentName }
    }
}
