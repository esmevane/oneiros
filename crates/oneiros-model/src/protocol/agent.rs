use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectAgentByName {
    #[serde(alias = "agent")]
    pub name: AgentName,
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: AgentName,
    pub persona: PersonaName,
    #[serde(default)]
    pub description: Description,
    #[serde(default)]
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentRequest {
    #[serde(default)]
    pub name: AgentName,
    pub persona: PersonaName,
    #[serde(default)]
    pub description: Description,
    #[serde(default)]
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentRequest {
    pub name: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveAgentRequest {
    pub name: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentsRequest;

// ── Protocol enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentEvents {
    AgentCreated(Agent),
    AgentUpdated(Agent),
    AgentRemoved(SelectAgentByName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentRequests {
    CreateAgent(CreateAgentRequest),
    UpdateAgent(UpdateAgentRequest),
    RemoveAgent(RemoveAgentRequest),
    GetAgent(GetAgentRequest),
    ListAgents(ListAgentsRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentResponses {
    AgentCreated(Agent),
    AgentUpdated(Agent),
    AgentFound(Agent),
    AgentsListed(Vec<Agent>),
    AgentRemoved,
}
