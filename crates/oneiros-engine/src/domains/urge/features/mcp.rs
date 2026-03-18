//! Urge MCP driving adapter — translates tool calls into domain service calls.

use crate::contexts::ProjectContext;
use crate::mcp_support::ToolError;

use super::super::model::Urge;
use super::super::service::UrgeService;

#[derive(serde::Deserialize)]
struct NameParam {
    name: String,
}

pub fn tool_names() -> &'static [&'static str] {
    &["set_urge", "get_urge", "list_urges", "remove_urge"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "set_urge" => {
            let urge: Urge =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                UrgeService::set(ctx, urge).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_urge" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                UrgeService::get(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_urges" => {
            let response = UrgeService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "remove_urge" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                UrgeService::remove(ctx, &p.name).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
