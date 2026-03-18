use serde::{Deserialize, Serialize};

use super::model::Agent;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentResponse {
    Created(Agent),
    Found(Agent),
    Listed(Vec<Agent>),
    Updated(Agent),
    Removed,
}
