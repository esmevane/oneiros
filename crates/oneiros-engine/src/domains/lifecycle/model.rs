use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// The full cognitive context for an agent — assembled by dream/introspect.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CognitiveContext {
    pub agent: Agent,
    #[serde(default)]
    pub persona: Option<Persona>,
    #[serde(default)]
    pub cognitions: Vec<Cognition>,
    #[serde(default)]
    pub memories: Vec<Memory>,
    #[serde(default)]
    pub experiences: Vec<Experience>,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub textures: Vec<Texture>,
    #[serde(default)]
    pub levels: Vec<Level>,
    #[serde(default)]
    pub sensations: Vec<Sensation>,
    #[serde(default)]
    pub natures: Vec<Nature>,
    #[serde(default)]
    pub urges: Vec<Urge>,
    #[serde(default, deserialize_with = "deserialize_pressures_tolerant")]
    pub pressures: Vec<Pressure>,
}

/// Tolerant deserializer for pressures — accepts the engine's Vec<Pressure> shape
/// or falls back to empty when the legacy's Vec<PressureReading> shape is present.
fn deserialize_pressures_tolerant<'de, D>(deserializer: D) -> Result<Vec<Pressure>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    // Try to deserialize as Vec<Pressure>; on any shape mismatch, return empty.
    let value = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(value).unwrap_or_default())
}

/// A lifecycle event marker — records that a lifecycle operation occurred.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LifecycleMarker {
    pub agent: AgentName,
    pub operation: Label,
    pub created_at: Timestamp,
}
