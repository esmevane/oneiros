use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SensationRequest {
    Set(Sensation),
    Get { name: SensationName },
    List,
    Remove { name: SensationName },
}
