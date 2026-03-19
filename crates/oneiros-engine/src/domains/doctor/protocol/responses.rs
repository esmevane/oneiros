use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorResponse {
    Initialized,
    NotInitialized,
    DatabaseOk(String),
    EventLogReady(i64),
}
