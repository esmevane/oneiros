use serde::{Deserialize, Serialize};

use super::model::Brain;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BrainResponse {
    Created(Brain),
    Found(Brain),
    Listed(Vec<Brain>),
}
