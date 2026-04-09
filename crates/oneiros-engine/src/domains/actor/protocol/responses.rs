use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = ActorResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActorResponse {
    Created(Response<Actor>),
    Found(Response<Actor>),
    Listed(Listed<Response<Actor>>),
}
