use serde::{Deserialize, Serialize};

use super::model::Sensation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(Sensation),
    SensationRemoved(SensationRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensationRemoved {
    pub name: String,
}
