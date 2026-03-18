//! Sensation MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize)]
struct NameParam {
    name: String,
}

pub fn tool_names() -> &'static [&'static str] {
    &[
        "set_sensation",
        "get_sensation",
        "list_sensations",
        "remove_sensation",
    ]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "set_sensation" => {
            let sensation: Sensation =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = SensationService::set(ctx, sensation)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_sensation" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = SensationService::get(ctx, &p.name)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_sensations" => {
            let response =
                SensationService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "remove_sensation" => {
            let p: NameParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = SensationService::remove(ctx, &p.name)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
