pub mod tenant_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct CreateParams {
        name: TenantName,
    }

    #[derive(Deserialize, JsonSchema)]
    struct GetParams {
        id: TenantId,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_tenant",
                description: "Create a new tenant in the system",
                input_schema: schema_for::<CreateParams>,
            },
            ToolDef {
                name: "get_tenant",
                description: "Look up a specific tenant by ID",
                input_schema: schema_for::<GetParams>,
            },
            ToolDef {
                name: "list_tenants",
                description: "List all tenants in the system",
                input_schema: schema_for::<serde_json::Value>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["create_tenant", "get_tenant", "list_tenants"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_tenant" => {
                let p: CreateParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TenantService::create(&system, p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_tenant" => {
                let p: GetParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TenantService::get(&system, &p.id)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_tenants" => {
                let response = TenantService::list(&system)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
