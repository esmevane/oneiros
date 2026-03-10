use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SelectUrgeByName {
    pub name: UrgeName,
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetUrgeRequest {
    pub name: UrgeName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RemoveUrgeRequest {
    pub name: UrgeName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListUrgesRequest;

// ── Protocol enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum UrgeEvents {
    UrgeSet(Urge),
    UrgeRemoved(SelectUrgeByName),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum UrgeRequests {
    SetUrge(Urge),
    RemoveUrge(RemoveUrgeRequest),
    GetUrge(GetUrgeRequest),
    ListUrges(ListUrgesRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum UrgeResponses {
    UrgeSet(Urge),
    UrgeFound(Urge),
    UrgesListed(Vec<Urge>),
    UrgeRemoved,
}
