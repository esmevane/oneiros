use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentResponse {
    AgentCreated(AgentName),
    AgentDetails(Agent),
    Agents(Vec<Agent>),
    NoAgents,
    AgentUpdated(AgentName),
    AgentRemoved(AgentName),
}
