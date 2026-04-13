use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = FollowResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[expect(
    clippy::large_enum_variant,
    reason = "We can reduce the size of the Follow later"
)]
pub(crate) enum FollowResponse {
    Found(Response<Follow>),
    Listed(Listed<Response<Follow>>),
}
