use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(Sensation),
    SensationRemoved(SensationRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensationRemoved {
    pub name: SensationName,
}
