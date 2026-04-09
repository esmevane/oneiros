use crate::*;

pub struct TenantTools;

impl TenantTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        tenant_mcp::tool_defs()
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

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateTenant>::new(TenantRequestType::CreateTenant, "Create a new tenant").def(),
            Tool::<GetTenant>::new(
                TenantRequestType::GetTenant,
                "Look up a specific tenant by ID",
            )
            .def(),
            Tool::<ListTenants>::new(TenantRequestType::ListTenants, "List all tenants").def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: TenantRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        let value = match request_type {
            TenantRequestType::CreateTenant => {
                TenantService::create(&system, &serde_json::from_str(params)?).await
            }
            TenantRequestType::GetTenant => {
                TenantService::get(&system, &serde_json::from_str(params)?).await
            }
            TenantRequestType::ListTenants => {
                TenantService::list(&system, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
