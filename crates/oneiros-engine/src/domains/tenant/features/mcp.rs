use crate::*;

pub struct TenantTools;

impl TenantTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        tenant_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        tenant_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        tenant_mcp::dispatch(context, tool_name, params).await
    }
}

mod tenant_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_tenant",
                description: "Create a new tenant",
                input_schema: schema_for::<CreateTenant>,
            },
            ToolDef {
                name: "get_tenant",
                description: "Look up a specific tenant by ID",
                input_schema: schema_for::<GetTenant>,
            },
            ToolDef {
                name: "list_tenants",
                description: "List all tenants",
                input_schema: schema_for::<ListTenants>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["create_tenant", "get_tenant", "list_tenants"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_tenant" => TenantService::create(&system, &serde_json::from_str(params)?).await,
            "get_tenant" => TenantService::get(&system, &serde_json::from_str(params)?).await,
            "list_tenants" => TenantService::list(&system, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
