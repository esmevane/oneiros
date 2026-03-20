use serde::{Deserialize, Serialize};

/// A single diagnostic check item emitted during a doctor checkup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorCheck {
    Initialized,
    NotInitialized,
    DatabaseOk(String),
    EventLogReady(i64),
}

/// All responses the doctor domain can produce.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorResponse {
    CheckupStatus(Vec<DoctorCheck>),
}
