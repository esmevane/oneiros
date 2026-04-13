use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = BrainResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum BrainResponse {
    Created(Response<Brain>),
    Found(Response<Brain>),
    Listed(Listed<Response<Brain>>),
}
