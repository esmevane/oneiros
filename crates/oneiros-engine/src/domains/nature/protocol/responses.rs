use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum NatureResponse {
    NatureSet(NatureName),
    NatureDetails(Nature),
    Natures(Vec<Nature>),
    NoNatures,
    NatureRemoved(NatureName),
}
