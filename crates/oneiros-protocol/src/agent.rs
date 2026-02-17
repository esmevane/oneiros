use oneiros_model::{Agent, AgentName, Description, PersonaName, Prompt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentEvents {
    AgentCreated(Agent),
    AgentUpdated(Agent),
    AgentRemoved { name: AgentName },
}

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
    pub persona: PersonaName,
    #[serde(default)]
    pub description: Description,
    #[serde(default)]
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentRequests {
    CreateAgent(CreateAgentRequest),
    UpdateAgent(UpdateAgentRequest),
    RemoveAgent { name: AgentName },
}
