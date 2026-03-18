use serde::{Deserialize, Serialize};

use super::model::Cognition;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionResponse {
    Added(Cognition),
    Found(Cognition),
    Listed(Vec<Cognition>),
}
