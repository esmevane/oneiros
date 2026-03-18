use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureEvents {
    NatureSet(Nature),
    NatureRemoved(NatureRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatureRemoved {
    pub name: NatureName,
}
