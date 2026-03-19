use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LifecycleResponse {
    Waking(CognitiveContext),
    Dreaming(CognitiveContext),
    Introspecting(CognitiveContext),
    Reflecting(CognitiveContext),
    Sleeping { agent: String },
    Guidebook(CognitiveContext),
}
