use oneiros_model::{Brain, BrainName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainEvents {
    BrainCreated(Brain),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBrainRequest {
    pub name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainRequests {
    CreateBrain(CreateBrainRequest),
}
