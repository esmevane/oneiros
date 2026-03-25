pub mod connection_mcp {
    //! Connection MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct IdParam {
        id: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct CreateConnectionParams {
        from_ref: String,
        to_ref: String,
        nature: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct ListConnectionsParams {
        entity: Option<String>,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_connection",
                description: "Draw a line between two related things",
                input_schema: schema_for::<CreateConnectionParams>,
            },
            ToolDef {
                name: "get_connection",
                description: "Examine a specific connection",
                input_schema: schema_for::<IdParam>,
            },
            ToolDef {
                name: "list_connections",
                description: "See how things connect",
                input_schema: schema_for::<ListConnectionsParams>,
            },
            ToolDef {
                name: "remove_connection",
                description: "Remove a connection between two entities",
                input_schema: schema_for::<IdParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
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
                let p: CreateConnectionParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ConnectionService::create(context, p.from_ref, p.to_ref, p.nature)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_connection" => {
                let p: IdParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let id: ConnectionId =
                    p.id.parse()
                        .map_err(|e: IdParseError| ToolError::Parameter(e.to_string()))?;
                let response = ConnectionService::get(context, &id)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_connections" => {
                let p: ListConnectionsParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ConnectionService::list(context, p.entity.as_deref())
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_connection" => {
                let p: IdParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let id: ConnectionId =
                    p.id.parse()
                        .map_err(|e: IdParseError| ToolError::Parameter(e.to_string()))?;
                let response = ConnectionService::remove(context, &id)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
