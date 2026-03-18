use serde::{Deserialize, Serialize};

use super::model::Agent;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentEvents {
    AgentCreated(Agent),
    AgentUpdated(Agent),
    AgentRemoved(AgentRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRemoved {
    pub name: String,
}
