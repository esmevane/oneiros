//! CLI command slice for the resource POC.
//!
//! In the real system, CLI commands use clap::Args for parsing and
//! oneiros-outcomes for structured output. For the POC, we demonstrate
//! the dispatch shape: each resource provides command execution that
//! goes through Fulfill (via HttpScope, since CLI → Client → HTTP → Service).

use oneiros_model::*;
use oneiros_resource::Fulfill;

use crate::resource_agent::Agent;
use crate::resource_level::Level;
use crate::{HttpScope, HttpScopeError};

/// CLI output — what the terminal would display.
///
/// In the real system this would be `Outcomes<T>` from oneiros-outcomes.
#[derive(Debug)]
pub struct CliOutput {
    pub messages: Vec<String>,
}

impl CliOutput {
    fn message(msg: impl Into<String>) -> Self {
        Self {
            messages: vec![msg.into()],
        }
    }
}

// ── Agent CLI commands ──────────────────────────────────────────────

impl Agent {
    /// Execute a CLI command for Agent operations.
    ///
    /// In the real system, each command is a separate clap::Args struct
    /// with a `run()` method. Here we show the dispatch shape.
    ///
    /// Note: CLI commands go through HttpScope (the client), not ProjectScope.
    /// CLI → HttpScope.fulfill → HTTP → ServiceState → ProjectScope.fulfill → DB
    pub async fn cli_run(
        scope: &HttpScope,
        command: &str,
        args: AgentCliArgs,
    ) -> Result<CliOutput, HttpScopeError> {
        match command {
            "create" => {
                let request = AgentRequests::CreateAgent(CreateAgentRequest {
                    name: args.name.clone().expect("name required"),
                    persona: args.persona.clone().expect("persona required"),
                    description: args.description.clone().unwrap_or_default(),
                    prompt: args.prompt.clone().unwrap_or_default(),
                });
                let response = Fulfill::<Agent>::fulfill(scope, request).await?;
                match response {
                    AgentResponses::AgentCreated(agent) => Ok(CliOutput::message(format!(
                        "Agent '{}' created.",
                        agent.name
                    ))),
                    other => Ok(CliOutput::message(format!("{other:?}"))),
                }
            }
            "list" => {
                let response =
                    Fulfill::<Agent>::fulfill(scope, AgentRequests::ListAgents(ListAgentsRequest))
                        .await?;
                match response {
                    AgentResponses::AgentsListed(agents) => {
                        let msgs: Vec<String> = agents.iter().map(|a| a.name.to_string()).collect();
                        Ok(CliOutput { messages: msgs })
                    }
                    other => Ok(CliOutput::message(format!("{other:?}"))),
                }
            }
            "show" => {
                let name = args.name.clone().expect("name required");
                let response = Fulfill::<Agent>::fulfill(
                    scope,
                    AgentRequests::GetAgent(GetAgentRequest { name }),
                )
                .await?;
                match response {
                    AgentResponses::AgentFound(agent) => Ok(CliOutput::message(format!(
                        "{}: {}",
                        agent.name, agent.description
                    ))),
                    other => Ok(CliOutput::message(format!("{other:?}"))),
                }
            }
            _ => Ok(CliOutput::message(format!("Unknown command: {command}"))),
        }
    }
}

/// Simplified CLI args for Agent commands.
///
/// In the real system, each command has its own clap::Args struct.
#[derive(Debug, Clone, Default)]
pub struct AgentCliArgs {
    pub name: Option<AgentName>,
    pub persona: Option<PersonaName>,
    pub description: Option<Description>,
    pub prompt: Option<Prompt>,
}

// ── Level CLI commands ──────────────────────────────────────────────

impl Level {
    pub async fn cli_run(
        scope: &HttpScope,
        command: &str,
        args: LevelCliArgs,
    ) -> Result<CliOutput, HttpScopeError> {
        match command {
            "set" => {
                let level = oneiros_model::Level::init(
                    args.name.clone().expect("name required"),
                    args.description.clone().unwrap_or_default(),
                    args.prompt.clone().unwrap_or_default(),
                );
                let response =
                    Fulfill::<Level>::fulfill(scope, LevelRequests::SetLevel(level)).await?;
                match response {
                    LevelResponses::LevelSet(level) => {
                        Ok(CliOutput::message(format!("Level '{}' set.", level.name)))
                    }
                    other => Ok(CliOutput::message(format!("{other:?}"))),
                }
            }
            "list" => {
                let response =
                    Fulfill::<Level>::fulfill(scope, LevelRequests::ListLevels(ListLevelsRequest))
                        .await?;
                match response {
                    LevelResponses::LevelsListed(levels) => {
                        let msgs: Vec<String> = levels.iter().map(|l| l.name.to_string()).collect();
                        Ok(CliOutput { messages: msgs })
                    }
                    other => Ok(CliOutput::message(format!("{other:?}"))),
                }
            }
            _ => Ok(CliOutput::message(format!("Unknown command: {command}"))),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LevelCliArgs {
    pub name: Option<LevelName>,
    pub description: Option<Description>,
    pub prompt: Option<Prompt>,
}
