use clap::Args;
use oneiros_db::Database;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum CheckupError {}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorOutcomes {
    // --- System checks ---
    #[outcome(message("System is initialized."))]
    Initialized,
    #[outcome(
        message("System is not initialized. Run `oneiros system init`."),
        level = "error"
    )]
    NotInitialized,
    #[outcome(message("System database found at '{}'.", .0.display()))]
    DatabaseOk(PathBuf),
    #[outcome(message("System database not found at '{}': {1}", .0.display()), level = "error")]
    NoDatabaseFound(PathBuf, String),
    #[outcome(message("Event log is ready with {0} events."))]
    EventLogReady(usize),
    #[outcome(message("Event log error: {0}"), level = "error")]
    NoEventLog(String),
    #[outcome(message("Config file found at '{}'.", .0.display()))]
    ConfigOk(PathBuf),
    #[outcome(message("No config file at '{}' (using defaults).", .0.display()), level = "info")]
    NoConfigFound(PathBuf),

    // --- Service checks ---
    #[outcome(message("Service is running."))]
    ServiceRunning,
    #[outcome(message("Service is not running: {0}"), level = "error")]
    ServiceNotRunning(String),

    // --- Project checks ---
    #[outcome(message("Project '{0}' detected at '{}'.", .1.display()))]
    ProjectDetected(String, PathBuf),
    #[outcome(message("No project detected."), level = "warn")]
    NoProjectDetected,
    #[outcome(message("Brain exists for project '{0}'."))]
    BrainExists(String),
    #[outcome(
        message("No brain for project '{0}'. Run `oneiros project init`."),
        level = "error"
    )]
    NoBrain(String),
    #[outcome(message("Auth token is valid for project '{0}'."))]
    TokenValid(String),
    #[outcome(message("Auth token is missing for project '{0}'."), level = "error")]
    TokenMissing(String),
    #[outcome(
        message("Auth token is invalid for project '{0}': {1}"),
        level = "error"
    )]
    TokenInvalid(String, String),

    // --- MCP checks ---
    #[outcome(message("MCP config found at '{}'.", .0.display()))]
    McpConfigOk(PathBuf),
    #[outcome(
        message("No .mcp.json found. Run `oneiros project init` to generate one."),
        level = "warn"
    )]
    NoMcpConfig,
    #[outcome(message("MCP config has matching auth token."))]
    McpTokenMatches,
    #[outcome(
        message(
            "MCP config token does not match stored ticket. Regenerate with `oneiros project init`."
        ),
        level = "warn"
    )]
    McpTokenMismatch,
    #[outcome(message(".mcp.json is not readable: {0}"), level = "warn")]
    McpConfigUnreadable(String),
}

#[derive(Args, Clone)]
pub struct DoctorOp;

impl DoctorOp {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<DoctorOutcomes>, Vec<PressureSummary>), CheckupError> {
        let mut checks = Outcomes::new();

        // --- System checks ---
        if context.is_initialized() {
            checks.emit(DoctorOutcomes::Initialized);
        } else {
            checks.emit(DoctorOutcomes::NotInitialized);
        }

        match Database::open(context.db_path()) {
            Ok(store) => {
                checks.emit(DoctorOutcomes::DatabaseOk(context.db_path()));

                match store.event_count() {
                    Ok(count) => checks.emit(DoctorOutcomes::EventLogReady(count)),
                    Err(error) => checks.emit(DoctorOutcomes::NoEventLog(error.to_string())),
                };
            }
            Err(error) => {
                checks.emit(DoctorOutcomes::NoDatabaseFound(
                    context.db_path(),
                    error.to_string(),
                ));
            }
        }

        if context.config_path().exists() {
            checks.emit(DoctorOutcomes::ConfigOk(context.config_path()));
        } else {
            checks.emit(DoctorOutcomes::NoConfigFound(context.config_path()));
        }

        // --- Service checks ---
        let client = context.client();
        let service_running = match client.health().await {
            Ok(()) => {
                checks.emit(DoctorOutcomes::ServiceRunning);
                true
            }
            Err(error) => {
                checks.emit(DoctorOutcomes::ServiceNotRunning(error.to_string()));
                false
            }
        };

        // --- Project checks ---
        let project_name = if let Some(name) = context.project_name() {
            let root = context
                .project_root()
                .map(PathBuf::from)
                .unwrap_or_default();
            checks.emit(DoctorOutcomes::ProjectDetected(name.to_string(), root));
            Some(name.to_string())
        } else {
            checks.emit(DoctorOutcomes::NoProjectDetected);
            None
        };

        if let Some(ref name) = project_name {
            // Check brain exists (via ticket file as proxy).
            let ticket_path = context.ticket_path(name);

            if ticket_path.exists() {
                checks.emit(DoctorOutcomes::BrainExists(name.clone()));

                // Validate the token against the running service.
                match context.ticket_token() {
                    Ok(token) => {
                        if service_running {
                            // Use a lightweight authenticated request to verify the token.
                            match client.list_agents(&token).await {
                                Ok(_) => {
                                    checks.emit(DoctorOutcomes::TokenValid(name.clone()));
                                }
                                Err(error) => {
                                    checks.emit(DoctorOutcomes::TokenInvalid(
                                        name.clone(),
                                        error.to_string(),
                                    ));
                                }
                            }
                        }
                    }
                    Err(_) => {
                        checks.emit(DoctorOutcomes::TokenMissing(name.clone()));
                    }
                }
            } else {
                checks.emit(DoctorOutcomes::NoBrain(name.clone()));
            }

            // --- MCP checks ---
            if let Some(project_root) = context.project_root() {
                let mcp_path = project_root.join(".mcp.json");

                if mcp_path.exists() {
                    checks.emit(DoctorOutcomes::McpConfigOk(mcp_path.clone()));

                    // Check that the MCP config token matches the stored ticket.
                    if let Ok(token) = context.ticket_token() {
                        match std::fs::read_to_string(&mcp_path) {
                            Ok(mcp_contents) => {
                                if mcp_contents.contains(&token.0) {
                                    checks.emit(DoctorOutcomes::McpTokenMatches);
                                } else {
                                    checks.emit(DoctorOutcomes::McpTokenMismatch);
                                }
                            }
                            Err(error) => {
                                checks.emit(DoctorOutcomes::McpConfigUnreadable(error.to_string()));
                            }
                        }
                    }
                } else {
                    checks.emit(DoctorOutcomes::NoMcpConfig);
                }
            }
        }

        Ok((checks, vec![]))
    }
}
