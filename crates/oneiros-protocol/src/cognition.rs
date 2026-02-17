use oneiros_model::{AgentName, Cognition, Content, TextureName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionEvents {
    CognitionAdded(Cognition),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCognitionRequest {
    pub agent: AgentName,
    pub texture: TextureName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionRequests {
    AddCognition(AddCognitionRequest),
}
