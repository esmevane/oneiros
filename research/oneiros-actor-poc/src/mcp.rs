//! MCP tool surface — dispatches tool calls through actor handles.
//!
//! The registry holds actor handles. Tool methods send messages through them.
//! No rmcp macros in the POC — we prove the dispatch shape manually.

use oneiros_model::*;

use crate::agent::AgentError;
use crate::registry::Registry;

/// MCP tool call result — serialized JSON content.
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Unknown tool: {0}")]
    UnknownTool(String),

    #[error("Parameter error: {0}")]
    Parameter(String),

    #[error(transparent)]
    Agent(#[from] AgentError),

    #[error("Actor communication error: {0}")]
    ActorError(String),
}

impl ToolResult {
    fn json<T: serde::Serialize>(value: &T) -> Result<Self, ToolError> {
        let content =
            serde_json::to_string(value).map_err(|e| ToolError::Parameter(e.to_string()))?;
        Ok(Self { content })
    }
}

/// Dispatch an MCP tool call through the registry.
///
/// The registry provides actor handles. Each tool sends a message
/// to the appropriate actor and formats the response.
pub async fn dispatch_tool(
    registry: &Registry,
    tool_name: &str,
    params: &str,
) -> Result<ToolResult, ToolError> {
    match tool_name {
        "list_agents" => {
            let response = registry
                .agents
                .send(AgentRequests::ListAgents(ListAgentsRequest))
                .await
                .map_err(|e| ToolError::ActorError(e.to_string()))?
                .map_err(ToolError::Agent)?;
            ToolResult::json(&response)
        }
        "get_agent" => {
            let request: GetAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = registry
                .agents
                .send(AgentRequests::GetAgent(request))
                .await
                .map_err(|e| ToolError::ActorError(e.to_string()))?
                .map_err(ToolError::Agent)?;
            ToolResult::json(&response)
        }
        "create_agent" => {
            let request: CreateAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = registry
                .agents
                .send(AgentRequests::CreateAgent(request))
                .await
                .map_err(|e| ToolError::ActorError(e.to_string()))?
                .map_err(ToolError::Agent)?;
            ToolResult::json(&response)
        }
        "update_agent" => {
            let request: UpdateAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = registry
                .agents
                .send(AgentRequests::UpdateAgent(request))
                .await
                .map_err(|e| ToolError::ActorError(e.to_string()))?
                .map_err(ToolError::Agent)?;
            ToolResult::json(&response)
        }
        "remove_agent" => {
            let request: RemoveAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = registry
                .agents
                .send(AgentRequests::RemoveAgent(request))
                .await
                .map_err(|e| ToolError::ActorError(e.to_string()))?
                .map_err(ToolError::Agent)?;
            ToolResult::json(&response)
        }
        _ => Err(ToolError::UnknownTool(tool_name.to_string())),
    }
}
