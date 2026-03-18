use serde::{Deserialize, Serialize};

use super::model::Urge;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum UrgeEvents {
    UrgeSet(Urge),
    UrgeRemoved(UrgeRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgeRemoved {
    pub name: String,
}
