use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionResponse {
    CognitionAdded(Response<Cognition>),
    CognitionDetails(Response<Cognition>),
    Cognitions(Listed<Response<Cognition>>),
    NoCognitions,
}
