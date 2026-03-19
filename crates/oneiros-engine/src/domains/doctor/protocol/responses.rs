use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorResponse {
    Initialized,
    NotInitialized,
    DatabaseOk(String),
    EventLogReady(i64),
}
