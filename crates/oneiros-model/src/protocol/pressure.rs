use serde::{Deserialize, Serialize};

use crate::*;

// ── Request types ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetPressureRequest {
    pub agent: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListPressuresRequest;

// ── Protocol enums ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PressureRequests {
    GetPressure(GetPressureRequest),
    ListPressures(ListPressuresRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PressureResponses {
    PressureFound(Vec<Pressure>),
    PressuresListed(Vec<Pressure>),
}
