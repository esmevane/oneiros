use crate::*;

pub struct ConnectionTools;

impl ConnectionTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        connection_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        connection_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        connection_mcp::dispatch(context, tool_name, params).await
    }
}

mod connection_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_connection",
                description: "Draw a line between two related things",
                input_schema: schema_for::<CreateConnection>,
            },
            ToolDef {
                name: "get_connection",
                description: "Examine a specific connection",
                input_schema: schema_for::<GetConnection>,
            },
            ToolDef {
                name: "list_connections",
                description: "See how things connect",
                input_schema: schema_for::<ListConnections>,
            },
            ToolDef {
                name: "remove_connection",
                description: "Remove a connection",
                input_schema: schema_for::<RemoveConnection>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "create_connection",
            "get_connection",
            "list_connections",
            "remove_connection",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "create_connection" => {
                ConnectionService::create(context, &serde_json::from_str(params)?).await
            }
            "get_connection" => {
                ConnectionService::get(context, &serde_json::from_str(params)?).await
            }
            "list_connections" => {
                ConnectionService::list(context, &serde_json::from_str(params)?).await
            }
            "remove_connection" => {
                ConnectionService::remove(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
