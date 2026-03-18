//! Memory MCP driving adapter — translates tool calls into domain service calls.

use crate::contexts::ProjectContext;
use crate::mcp_support::ToolError;

use super::super::service::MemoryService;

#[derive(serde::Deserialize)]
struct IdParam {
    id: String,
}

#[derive(serde::Deserialize)]
struct AddMemoryParams {
    agent: String,
    level: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct ListMemoriesParams {
    agent: Option<String>,
}

pub fn tool_names() -> &'static [&'static str] {
    &["add_memory", "get_memory", "list_memories"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "add_memory" => {
            let p: AddMemoryParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = MemoryService::add(ctx, p.agent, p.level, p.content)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_memory" => {
            let p: IdParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                MemoryService::get(ctx, &p.id).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_memories" => {
            let p: ListMemoriesParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = MemoryService::list(ctx, p.agent.as_deref())
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
