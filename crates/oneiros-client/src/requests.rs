use oneiros_model::{
    AgentName, BrainName, Content, Description, LevelName, PersonaName, Prompt, TextureName,
};
use serde::{Deserialize, Serialize};

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
pub struct AddCognitionRequest {
    pub agent: AgentName,
    pub texture: TextureName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemoryRequest {
    pub agent: AgentName,
    pub level: LevelName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrainRequest {
    pub name: BrainName,
}
