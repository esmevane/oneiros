use serde::{Deserialize, Serialize};

/// A pressure reading — derived state computed from cross-domain queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pressure {
    pub agent: String,
    pub urge: String,
    pub percent: u8,
    pub updated_at: String,
}

/// Summary for wire format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureSummary {
    pub urge: String,
    pub percent: u8,
}
