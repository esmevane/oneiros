use oneiros_model::{Brain, BrainId, Identity, Token};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub entity: Identity<BrainId, Brain>,
    pub token: Token,
}
