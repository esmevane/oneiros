//! Level MCP driving adapter — translates tool calls into domain service calls.

use crate::contexts::ProjectContext;
use crate::mcp_support::ToolError;

use super::super::model::Level;
use super::super::service::LevelService;

#[derive(serde::Deserialize)]
struct NameParam {
    name: String,
}

pub fn tool_names() -> &'static [&'static str] {
    &["set_level", "get_level", "list_levels", "remove_level"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "set_level" => {
            let level: Level =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                LevelService::set(ctx, level).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_level" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                LevelService::get(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_levels" => {
            let response = LevelService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "remove_level" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                LevelService::remove(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
