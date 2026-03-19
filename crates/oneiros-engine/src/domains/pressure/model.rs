use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A pressure reading — derived state computed from cross-domain queries.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Pressure {
    pub agent: String,
    pub urge: String,
    pub percent: u8,
    pub updated_at: String,
}

/// Compact pressure summary for response metadata.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PressureSummary {
    pub urge: String,
    pub percent: u8,
}
