use oneiros_model::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub entity: Identity<BrainId, HasPath<Brain>>,
    pub token: Token,
}
