use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UrgeRequest {
    Set(Urge),
    Get { name: UrgeName },
    List,
    Remove { name: UrgeName },
}
