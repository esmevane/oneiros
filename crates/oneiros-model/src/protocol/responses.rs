use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainInfo {
    pub entity: BrainId,
    pub token: Token,
}
