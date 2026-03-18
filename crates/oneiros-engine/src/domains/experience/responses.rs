use serde::{Deserialize, Serialize};

use super::model::Experience;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExperienceResponse {
    Created(Experience),
    Found(Experience),
    Listed(Vec<Experience>),
    Updated(Experience),
}
