use oneiros_model::{
    AgentName, BrainName, Content, Description, Id, Label, LevelName, PersonaName, Prompt,
    RecordKind, RecordRef, SensationName, TextureName,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperienceRequest {
    pub agent: AgentName,
    pub sensation: SensationName,
    pub description: Content,
    #[serde(default)]
    pub refs: Vec<RecordRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddExperienceRefRequest {
    pub record_id: Id,
    pub record_kind: RecordKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Label>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperienceDescriptionRequest {
    pub description: Content,
}
