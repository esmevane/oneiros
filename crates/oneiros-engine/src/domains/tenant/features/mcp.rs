use crate::*;

pub struct TenantTools;

impl TenantTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        tenant_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        tenant_mcp::dispatch(state, config, tool_name, params).await
    }
}

mod tenant_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateTenant>::def(TenantRequestType::CreateTenant, "Create a new tenant"),
            Tool::<GetTenant>::def(
                TenantRequestType::GetTenant,
                "Look up a specific tenant by ID",
            ),
            Tool::<ListTenants>::def(TenantRequestType::ListTenants, "List all tenants"),
        ]
    }

    pub async fn dispatch(
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let context = state
            .project_context(config.clone())
            .map_err(|e| ToolError::Domain(e.to_string()))?;

        let request_type: TenantRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        match request_type {
            TenantRequestType::CreateTenant => {
                let resp = TenantService::create(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TenantResponse::Created(wrapped) => Ok(McpResponse::new(format!(
                        "Tenant created: {}",
                        wrapped.data.name
                    ))),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            TenantRequestType::GetTenant => {
                let resp = TenantService::get(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TenantResponse::Found(wrapped) => {
                        Ok(McpResponse::new(format!("**name:** {}", wrapped.data.name)))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            TenantRequestType::ListTenants => {
                let resp = TenantService::list(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TenantResponse::Listed(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
