//! MCP tool slice for the resource POC.
//!
//! rmcp's `#[tool_router]` macro requires all tools on one struct, so we
//! can't literally move tools onto the resource type. Instead, each resource
//! provides a `handle_tool` function that the toolbox delegates to.
//!
//! The pattern: resource owns the tool logic, the toolbox is just a router.
//!
//! This module doesn't use rmcp macros (to avoid the dep in the POC).
//! Instead it demonstrates the dispatch shape manually.

use oneiros_model::*;
use oneiros_resource::{Feature, Tools};

use crate::resource_agent::Agent;
use crate::resource_level::Level;
use crate::{ServiceState, ServiceStateError};

/// The output of an MCP tool call — serialized JSON content.
///
/// In the real system this would be `rmcp::model::CallToolResult`.
/// For the POC, raw JSON is sufficient to prove the dispatch shape.
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: String,
}

impl ToolResult {
    fn json<T: serde::Serialize>(value: &T) -> Result<Self, ToolError> {
        let content = serde_json::to_string(value)
            .map_err(|e| ToolError::Serialization(e.to_string()))?;
        Ok(Self { content })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Unknown tool: {0}")]
    UnknownTool(String),

    #[error("Parameter error: {0}")]
    Parameter(String),

    #[error(transparent)]
    Service(#[from] ServiceStateError),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// The surface produced by Feature<Tools> — tool names + handler.
///
/// The handler is a function pointer that takes state, tool name, and
/// params and returns a result. This is what the AppBuilder collects
/// to build a unified MCP tool router.
pub struct ToolSurface {
    pub names: &'static [&'static str],
    pub handler: fn(&ServiceState, &str, &str) -> Result<ToolResult, ToolError>,
}

// ── Feature<Tools> ──────────────────────────────────────────────────

impl Feature<Tools> for Agent {
    type Surface = ToolSurface;

    fn feature(&self) -> ToolSurface {
        ToolSurface {
            names: Self::tool_names(),
            handler: Self::handle_tool,
        }
    }
}

impl Feature<Tools> for Level {
    type Surface = ToolSurface;

    fn feature(&self) -> ToolSurface {
        ToolSurface {
            names: Self::tool_names(),
            handler: Self::handle_tool,
        }
    }
}

// ── Agent MCP tools ─────────────────────────────────────────────────

impl Agent {
    /// Handle an MCP tool call for Agent operations.
    ///
    /// In the real system, rmcp's `#[tool]` macro generates this routing.
    /// Here we demonstrate that the resource can own the tool dispatch logic.
    pub fn handle_tool(
        state: &ServiceState,
        tool_name: &str,
        params: &str,
    ) -> Result<ToolResult, ToolError> {
        match tool_name {
            "list_agents" => {
                let response = state.fulfill::<Agent>(
                    AgentRequests::ListAgents(ListAgentsRequest),
                )?;
                ToolResult::json(&response)
            }
            "get_agent" => {
                let request: GetAgentRequest = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Agent>(
                    AgentRequests::GetAgent(request),
                )?;
                ToolResult::json(&response)
            }
            "create_agent" => {
                let request: CreateAgentRequest = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Agent>(
                    AgentRequests::CreateAgent(request),
                )?;
                ToolResult::json(&response)
            }
            "update_agent" => {
                let request: UpdateAgentRequest = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Agent>(
                    AgentRequests::UpdateAgent(request),
                )?;
                ToolResult::json(&response)
            }
            "remove_agent" => {
                let request: RemoveAgentRequest = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Agent>(
                    AgentRequests::RemoveAgent(request),
                )?;
                ToolResult::json(&response)
            }
            _ => Err(ToolError::UnknownTool(tool_name.to_string())),
        }
    }

    /// Tool names this resource provides. In production, derived from
    /// the rmcp #[tool] attributes.
    pub fn tool_names() -> &'static [&'static str] {
        &["list_agents", "get_agent", "create_agent", "update_agent", "remove_agent"]
    }
}

// ── Level MCP tools ─────────────────────────────────────────────────

impl Level {
    pub fn handle_tool(
        state: &ServiceState,
        tool_name: &str,
        params: &str,
    ) -> Result<ToolResult, ToolError> {
        match tool_name {
            "list_levels" => {
                let response = state.fulfill::<Level>(
                    LevelRequests::ListLevels(ListLevelsRequest),
                )?;
                ToolResult::json(&response)
            }
            "get_level" => {
                let request: GetLevelRequest = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Level>(
                    LevelRequests::GetLevel(request),
                )?;
                ToolResult::json(&response)
            }
            "set_level" => {
                let level: oneiros_model::Level = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Level>(
                    LevelRequests::SetLevel(level),
                )?;
                ToolResult::json(&response)
            }
            "remove_level" => {
                let request: RemoveLevelRequest = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = state.fulfill::<Level>(
                    LevelRequests::RemoveLevel(request),
                )?;
                ToolResult::json(&response)
            }
            _ => Err(ToolError::UnknownTool(tool_name.to_string())),
        }
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["list_levels", "get_level", "set_level", "remove_level"]
    }
}

/// Route a tool call to the right resource handler.
///
/// This is what the rmcp ToolRouter does internally. In the resource model,
/// each resource registers its tool names, and the router delegates.
pub fn dispatch_tool(
    state: &ServiceState,
    tool_name: &str,
    params: &str,
) -> Result<ToolResult, ToolError> {
    // Check agent tools first, then level tools
    if Agent::tool_names().contains(&tool_name) {
        Agent::handle_tool(state, tool_name, params)
    } else if Level::tool_names().contains(&tool_name) {
        Level::handle_tool(state, tool_name, params)
    } else {
        Err(ToolError::UnknownTool(tool_name.to_string()))
    }
}
