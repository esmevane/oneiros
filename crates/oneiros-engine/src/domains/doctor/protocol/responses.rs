use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

/// The filename or path label identifying which database was checked.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct DatabaseLabel(pub(crate) String);

impl DatabaseLabel {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl core::fmt::Display for DatabaseLabel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// The number of events present in the event log.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct LogEventCount(pub(crate) i64);

impl LogEventCount {
    pub(crate) fn new(value: i64) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for LogEventCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// A single diagnostic check item emitted during a doctor checkup.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = DoctorCheckType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum DoctorCheck {
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
    McpConfigured,
    McpMissing,
    ServiceRunning,
    ServiceStopped,
    ServiceNotInstalled,
}

/// All responses the doctor domain can produce.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = DoctorResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum DoctorResponse {
    CheckupStatus(Vec<DoctorCheck>),
}
