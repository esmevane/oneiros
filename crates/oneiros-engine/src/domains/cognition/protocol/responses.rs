use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = CognitionResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum CognitionResponse {
    CognitionAdded(Response<Cognition>),
    CognitionDetails(Response<Cognition>),
    Cognitions(Listed<Response<Cognition>>),
    NoCognitions,
}
