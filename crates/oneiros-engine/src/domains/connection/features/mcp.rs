//! Connection MCP driving adapter — translates tool calls into domain service calls.

use crate::contexts::ProjectContext;
use crate::mcp_support::ToolError;

use super::super::service::ConnectionService;

#[derive(serde::Deserialize)]
struct IdParam {
    id: String,
}

#[derive(serde::Deserialize)]
struct CreateConnectionParams {
    from_entity: String,
    to_entity: String,
    nature: String,
    description: String,
}

#[derive(serde::Deserialize)]
struct ListConnectionsParams {
    entity: Option<String>,
}

pub fn tool_names() -> &'static [&'static str] {
    &[
        "create_connection",
        "get_connection",
        "list_connections",
        "remove_connection",
    ]
}

pub fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    let value = match tool_name {
        "create_connection" => {
            let p: CreateConnectionParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                ConnectionService::create(ctx, p.from_entity, p.to_entity, p.nature, p.description)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "get_connection" => {
            let p: IdParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response =
                ConnectionService::get(ctx, &p.id).map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "list_connections" => {
            let p: ListConnectionsParams =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = ConnectionService::list(ctx, p.entity.as_deref())
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        "remove_connection" => {
            let p: IdParam =
                serde_json::from_str(params).map_err(|e| ToolError::Parameter(e.to_string()))?;
            let response = ConnectionService::remove(ctx, &p.id)
                .map_err(|e| ToolError::Domain(e.to_string()))?;
            serde_json::to_value(response)
        }
        _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
    };
    value.map_err(|e| ToolError::Parameter(e.to_string()))
}
