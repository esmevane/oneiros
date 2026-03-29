use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureResult {
    pub agent: AgentName,
    pub pressures: Vec<Pressure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PressureResponse {
    Readings(PressureResult),
}
