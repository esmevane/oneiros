use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCognitionById {
    pub id: CognitionId,
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCognitionRequest {
    pub agent: AgentName,
    pub texture: TextureName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCognitionRequest {
    pub id: CognitionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCognitionsRequest {
    #[serde(default)]
    pub agent: Option<AgentName>,
    #[serde(default)]
    pub texture: Option<TextureName>,
}

// ── Protocol enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionEvents {
    CognitionAdded(Cognition),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionRequests {
    AddCognition(AddCognitionRequest),
    GetCognition(GetCognitionRequest),
    ListCognitions(ListCognitionsRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionResponses {
    CognitionAdded(Cognition),
    CognitionFound(Cognition),
    CognitionsListed(Vec<Cognition>),
}
