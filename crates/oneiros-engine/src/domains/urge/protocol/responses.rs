use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = UrgeResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum UrgeResponse {
    UrgeSet(UrgeName),
    UrgeDetails(Urge),
    Urges(Listed<Urge>),
    NoUrges,
    UrgeRemoved(UrgeName),
}
