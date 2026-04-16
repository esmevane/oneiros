use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = UrgeResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UrgeResponse {
    UrgeSet(UrgeName),
    UrgeDetails(Urge),
    Urges(Listed<Urge>),
    NoUrges,
    UrgeRemoved(UrgeName),
}
