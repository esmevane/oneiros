use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SensationResponse {
    SensationSet(SensationName),
    SensationDetails(Response<Sensation>),
    Sensations(Listed<Response<Sensation>>),
    NoSensations,
    SensationRemoved(SensationName),
}
