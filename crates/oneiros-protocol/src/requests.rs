use oneiros_link::*;
use oneiros_model::*;
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
    pub description: Description,
    #[serde(default)]
    pub refs: Vec<RecordRef>,
}

/// A request to add a reference to an experience.
///
/// Accepts either form via serde untagged:
///   Identified: { "record_id": "...", "record_kind": "cognition", "role": "origin" }
///   Linked:     { "link": "base64url...", "role": "origin" }
pub type AddExperienceRefRequest = RecordRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub nature: NatureName,
    pub from_link: Link,
    pub to_link: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperienceDescriptionRequest {
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensingRequests {
    Sense { agent: oneiros_model::AgentName },
}
