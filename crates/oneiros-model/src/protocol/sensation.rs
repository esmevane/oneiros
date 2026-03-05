use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectSensationByName {
    pub name: SensationName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(Sensation),
    SensationRemoved(SelectSensationByName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationRequests {
    SetSensation(Sensation),
    RemoveSensation(SelectSensationByName),
    GetSensation(SelectSensationByName),
    ListSensations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationResponses {
    SensationSet(Sensation),
    SensationFound(Sensation),
    SensationsListed(Vec<Sensation>),
    SensationRemoved,
}
