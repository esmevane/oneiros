use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = AgentResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentResponse {
    AgentCreated(AgentCreatedResponse),
    AgentDetails(AgentDetailsResponse),
    Agents(AgentsResponse),
    NoAgents,
    AgentUpdated(AgentUpdatedResponse),
    AgentRemoved(AgentRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum AgentCreatedResponse {
        V1 => { #[serde(flatten)] pub agent: Agent }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum AgentDetailsResponse {
        V1 => { #[serde(flatten)] pub agent: Agent }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum AgentsResponse {
        V1 => { pub items: Vec<Agent>, pub total: usize }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum AgentUpdatedResponse {
        V1 => { #[serde(flatten)] pub agent: Agent }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum AgentRemovedResponse {
        V1 => { #[builder(into)] pub name: AgentName }
    }
}
