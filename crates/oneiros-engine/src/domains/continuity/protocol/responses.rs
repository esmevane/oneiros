use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = ContinuityResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ContinuityResponse {
    Emerged(DreamContext),
    Waking(DreamContext),
    Dreaming(DreamContext),
    Introspecting(DreamContext),
    Reflecting(DreamContext),
    Sleeping(DreamContext),
    Receded(AgentName),
    Status(AgentActivityTable),
    Guidebook(DreamContext),
}
