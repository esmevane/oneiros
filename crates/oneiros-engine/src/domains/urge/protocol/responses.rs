use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UrgeResponse {
    UrgeSet(UrgeName),
    UrgeDetails(Urge),
    Urges(Vec<Urge>),
    NoUrges,
    UrgeRemoved(UrgeName),
}
