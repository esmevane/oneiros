use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectCognitionById {
    pub id: CognitionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCognitionsFilter {
    #[serde(default)]
    pub agent: Option<AgentName>,
    #[serde(default)]
    pub texture: Option<TextureName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    GetCognition(SelectCognitionById),
    ListCognitions(ListCognitionsFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionResponses {
    CognitionAdded(Cognition),
    CognitionFound(Cognition),
    CognitionsListed(Vec<Cognition>),
}
