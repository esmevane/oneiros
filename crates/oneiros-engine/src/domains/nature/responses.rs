use serde::{Deserialize, Serialize};

use super::model::Nature;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum NatureResponse {
    Set(Nature),
    Found(Nature),
    Listed(Vec<Nature>),
    Removed,
}
