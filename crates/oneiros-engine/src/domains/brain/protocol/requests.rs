use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BrainRequest {
    Create { name: BrainName },
    Get { name: BrainName },
    List,
}
