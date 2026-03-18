use serde::{Deserialize, Serialize};

use super::model::Urge;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UrgeResponse {
    Set(Urge),
    Found(Urge),
    Listed(Vec<Urge>),
    Removed,
}
