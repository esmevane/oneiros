pub mod brain_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct CreateParams {
        name: BrainName,
    }

    #[derive(Deserialize, JsonSchema)]
    struct GetParams {
        name: BrainName,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_brain",
                description: "Create a new brain in the system",
                input_schema: schema_for::<CreateParams>,
            },
            ToolDef {
                name: "get_brain",
                description: "Look up a specific brain by name",
                input_schema: schema_for::<GetParams>,
            },
            ToolDef {
                name: "list_brains",
                description: "List all brains in the system",
                input_schema: schema_for::<serde_json::Value>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["create_brain", "get_brain", "list_brains"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_brain" => {
                let p: CreateParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = BrainService::create(&system, p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_brain" => {
                let p: GetParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = BrainService::get(&system, &p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_brains" => {
                let response = BrainService::list(&system)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
