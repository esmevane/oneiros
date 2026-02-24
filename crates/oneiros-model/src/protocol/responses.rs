use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub entity: Identity<BrainId, HasPath<Brain>>,
    pub token: Token,
}
