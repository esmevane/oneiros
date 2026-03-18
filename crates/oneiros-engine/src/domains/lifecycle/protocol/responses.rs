use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LifecycleResponse {
    Dreamed(CognitiveContext),
    Introspected(CognitiveContext),
    Reflected(CognitiveContext),
    Sensed { agent: String },
    Slept { agent: String },
}
