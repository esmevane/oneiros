//! Search MCP driving adapter — translates tool calls into domain service calls.

use crate::*;

#[derive(serde::Deserialize)]
struct SearchParams {
    query: String,
    agent: Option<String>,
}

pub fn tool_names() -> &'static [&'static str] {
    &["search"]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "search" => {
            let p: SearchParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = SearchService::search(ctx, &p.query, p.agent.as_deref())
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
