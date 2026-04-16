use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = SensationResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SensationResponse {
    SensationSet(SensationName),
    SensationDetails(Response<Sensation>),
    Sensations(Listed<Response<Sensation>>),
    NoSensations,
    SensationRemoved(SensationName),
}
