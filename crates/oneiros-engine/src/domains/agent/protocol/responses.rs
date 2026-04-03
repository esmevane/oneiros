use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentResponse {
    AgentCreated(AgentName),
    AgentDetails(Response<Agent>),
    Agents(Listed<Response<Agent>>),
    NoAgents,
    AgentUpdated(AgentName),
    AgentRemoved(AgentName),
}
