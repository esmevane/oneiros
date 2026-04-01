use serde::{Deserialize, Serialize};

use crate::*;

/// A single step in the setup flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetupStep {
    SystemInitialized,
    SystemAlreadyInitialized,
    ProjectInitialized(BrainName),
    ProjectAlreadyExists(BrainName),
    VocabularySeeded,
    AgentsSeeded,
    McpConfigured,
    McpSkipped,
    ServiceInstalled,
    ServiceStarted,
    ServiceSkipped,
    StepFailed { step: String, reason: String },
}

/// The result of running setup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetupResponse {
    SetupComplete(Vec<SetupStep>),
}
