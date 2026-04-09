use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureResult {
    pub agent: AgentName,
    pub pressures: Vec<Pressure>,
}

/// Pressure readings for all agents — returned by list queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPressureResult {
    pub pressures: Vec<Pressure>,
}

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = PressureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PressureResponse {
    Readings(PressureResult),
    AllReadings(ListPressureResult),
}
