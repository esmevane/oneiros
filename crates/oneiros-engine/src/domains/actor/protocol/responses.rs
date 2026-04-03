use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActorResponse {
    Created(Response<Actor>),
    Found(Response<Actor>),
    Listed(Listed<Response<Actor>>),
}
