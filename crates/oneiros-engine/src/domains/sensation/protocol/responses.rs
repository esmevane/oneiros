use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SensationResponse {
    Set(Sensation),
    Found(Sensation),
    Listed(Vec<Sensation>),
    Removed,
}
