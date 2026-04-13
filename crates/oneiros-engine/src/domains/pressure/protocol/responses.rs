use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PressureResult {
    pub(crate) agent: AgentName,
    pub(crate) pressures: Vec<Pressure>,
}

/// Pressure readings for all agents — returned by list queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ListPressureResult {
    pub(crate) pressures: Vec<Pressure>,
}

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = PressureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum PressureResponse {
    Readings(PressureResult),
    AllReadings(ListPressureResult),
}
