use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

/// The filename or path label identifying which database was checked.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(transparent)]
pub struct DatabaseLabel(pub String);

impl DatabaseLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl core::fmt::Display for DatabaseLabel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// The number of events present in the event log.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(transparent)]
pub struct LogEventCount(pub i64);

impl LogEventCount {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for LogEventCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// A single diagnostic check item emitted during a doctor checkup.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = DoctorCheckType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorCheck {
    Initialized,
    NotInitialized,
    DatabaseOk(DatabaseLabel),
    EventLogReady(LogEventCount),
    BrainExists(BrainName),
    BrainMissing(BrainName),
    VocabularySeeded,
    VocabularyMissing,
    AgentsSeeded,
    AgentsMissing,
    HostKeyOk,
    HostKeyMissing,
    McpConfigured,
    McpMissing,
    ServiceRunning,
    ServiceStopped,
    ServiceNotInstalled,
}

/// All responses the doctor domain can produce.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = DoctorResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorResponse {
    CheckupStatus(CheckupStatusResponse),
}

versioned! {
    #[derive(schemars::JsonSchema)]
    pub enum CheckupStatusResponse {
        V1 => {
            pub checks: Vec<DoctorCheck>,
        }
    }
}
