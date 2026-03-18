//! Cognition MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize)]
struct IdParam {
    id: String,
}

#[derive(serde::Deserialize)]
struct AddCognitionParams {
    agent: String,
    texture: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct ListCognitionsParams {
    agent: Option<String>,
    texture: Option<String>,
}

pub fn tool_names() -> &'static [&'static str] {
    &["add_cognition", "get_cognition", "list_cognitions"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "add_cognition" => {
            let p: AddCognitionParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = CognitionService::add(ctx, p.agent, p.texture, p.content)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_cognition" => {
            let p: IdParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                CognitionService::get(ctx, &p.id).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_cognitions" => {
            let p: ListCognitionsParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = CognitionService::list(ctx, p.agent.as_deref(), p.texture.as_deref())
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
