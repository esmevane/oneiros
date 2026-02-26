use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(Sensation),
    SensationRemoved { name: SensationName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationRequests {
    SetSensation(Sensation),
    RemoveSensation { name: SensationName },
    GetSensation { name: SensationName },
    ListSensations,
}
