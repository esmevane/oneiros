use serde::{Deserialize, Serialize};

use super::model::Sensation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SensationRequest {
    Set(Sensation),
    Get { name: String },
    List,
    Remove { name: String },
}
