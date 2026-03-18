use serde::{Deserialize, Serialize};

use super::model::Nature;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum NatureRequest {
    Set(Nature),
    Get { name: String },
    List,
    Remove { name: String },
}
