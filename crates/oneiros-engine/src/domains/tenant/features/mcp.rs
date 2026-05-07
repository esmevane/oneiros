use crate::*;

pub(crate) struct TenantMcp;

impl TenantMcp {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        tenant_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        tenant_mcp::dispatch(context, mailbox, tool_name, params).await
    }
}

mod tenant_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
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

    pub(crate) async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: TenantRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let scope = ComposeScope::new(context.config.clone())
            .host()
            .map_err(Error::from)?;

        let value = match request_type {
            TenantRequestType::CreateTenant => {
                TenantService::create(&scope, mailbox, &serde_json::from_str(params)?).await
            }
            TenantRequestType::GetTenant => {
                TenantService::get(&scope, &serde_json::from_str(params)?).await
            }
            TenantRequestType::ListTenants => {
                TenantService::list(&scope, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
