use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureEvents {
    NatureSet(Nature),
    NatureRemoved { name: NatureName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureRequests {
    SetNature(Nature),
    RemoveNature { name: NatureName },
    GetNature { name: NatureName },
    ListNatures,
}
