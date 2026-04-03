use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BrainResponse {
    Created(Response<Brain>),
    Found(Response<Brain>),
    Listed(Listed<Response<Brain>>),
}
