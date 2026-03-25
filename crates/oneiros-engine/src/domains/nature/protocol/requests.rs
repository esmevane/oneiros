use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum NatureRequest {
    Set(Nature),
    Get { name: NatureName },
    List,
    Remove { name: NatureName },
}
