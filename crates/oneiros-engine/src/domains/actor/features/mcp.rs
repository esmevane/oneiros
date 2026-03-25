pub mod actor_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct CreateParams {
        tenant_id: TenantId,
        name: ActorName,
    }

    #[derive(Deserialize, JsonSchema)]
    struct GetParams {
        id: ActorId,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_actor",
                description: "Create a new actor in the system",
                input_schema: schema_for::<CreateParams>,
            },
            ToolDef {
                name: "get_actor",
                description: "Look up a specific actor by ID",
                input_schema: schema_for::<GetParams>,
            },
            ToolDef {
                name: "list_actors",
                description: "List all actors in the system",
                input_schema: schema_for::<serde_json::Value>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["create_actor", "get_actor", "list_actors"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_actor" => {
                let p: CreateParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ActorService::create(&system, p.tenant_id, p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_actor" => {
                let p: GetParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ActorService::get(&system, p.id)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_actors" => {
                let response = ActorService::list(&system)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
