use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainEvents {
    BrainCreated(Brain),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrainRequest {
    pub name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub entity: BrainId,
    pub token: Token,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectBrainByName {
    pub name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBrainRequest {
    pub name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBrainsRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainRequests {
    CreateBrain(CreateBrainRequest),
    GetBrain(GetBrainRequest),
    ListBrains(ListBrainsRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainSummary {
    pub agents: Vec<Agent>,
    pub cognition_count: usize,
    pub memory_count: usize,
    pub experience_count: usize,
    pub connection_count: usize,
    pub event_count: usize,
    pub recent_cognitions: Vec<Cognition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainResponses {
    BrainCreated(BrainInfo),
    BrainFound(Brain),
    BrainsListed(Vec<Brain>),
    BrainSummarized(BrainSummary),
}
