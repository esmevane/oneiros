use serde::{Deserialize, Serialize};

use super::model::Urge;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UrgeRequest {
    Set(Urge),
    Get { name: String },
    List,
    Remove { name: String },
}
