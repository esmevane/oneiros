//! Agent MCP driving adapter — translates tool calls into domain service calls.

use oneiros_model::*;

use super::{AgentError, AgentService};
use crate::ports::AppContext;

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Unknown tool: {0}")]
    UnknownTool(String),

    #[error("Parameter error: {0}")]
    Parameter(String),

    #[error(transparent)]
    Domain(#[from] AgentError),
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: String,
}

pub fn tool_names() -> &'static [&'static str] {
    &[
        "list_agents",
        "get_agent",
        "create_agent",
        "update_agent",
        "remove_agent",
    ]
}

pub fn dispatch(ctx: &AppContext, tool_name: &str, params: &str) -> Result<ToolResult, ToolError> {
    let response = match tool_name {
        "list_agents" => AgentService::list(ctx)?,
        "get_agent" => {
            let req: GetAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            AgentService::get(ctx, &req.name)?
        }
        "create_agent" => {
            let req: CreateAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            AgentService::create(ctx, req)?
        }
        "update_agent" => {
            let req: UpdateAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            AgentService::update(ctx, req)?
        }
        "remove_agent" => {
            let req: RemoveAgentRequest =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            AgentService::remove(ctx, req.name)?
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };

    let content =
        serde_json::to_string(&response).map_err(|e| ToolError::Parameter(e.to_string()))?;
    Ok(ToolResult { content })
}
