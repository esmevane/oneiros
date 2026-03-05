use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectNatureByName {
    pub name: NatureName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureEvents {
    NatureSet(Nature),
    NatureRemoved(SelectNatureByName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureRequests {
    SetNature(Nature),
    RemoveNature(SelectNatureByName),
    GetNature(SelectNatureByName),
    ListNatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureResponses {
    NatureSet(Nature),
    NatureFound(Nature),
    NaturesListed(Vec<Nature>),
    NatureRemoved,
}
