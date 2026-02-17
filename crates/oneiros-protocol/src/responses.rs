use oneiros_model::{Brain, Token};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub entity: Brain,
    pub token: Token,
}
