use crate::*;

pub struct BrainTools;

impl BrainTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        brain_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        brain_mcp::dispatch(state, config, tool_name, params).await
    }
}

mod brain_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateBrain>::def(BrainRequestType::CreateBrain, "Create a new brain"),
            Tool::<GetBrain>::def(
                BrainRequestType::GetBrain,
                "Look up a specific brain by name",
            ),
            Tool::<ListBrains>::def(BrainRequestType::ListBrains, "List all brains"),
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

        let request_type: BrainRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        match request_type {
            BrainRequestType::CreateBrain => {
                let resp = BrainService::create(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BrainResponse::Created(wrapped) => Ok(McpResponse::new(format!(
                        "Brain created: {}",
                        wrapped.data.name
                    ))),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            BrainRequestType::GetBrain => {
                let resp = BrainService::get(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BrainResponse::Found(wrapped) => {
                        Ok(McpResponse::new(format!("**name:** {}", wrapped.data.name)))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            BrainRequestType::ListBrains => {
                let resp = BrainService::list(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    BrainResponse::Listed(listed) => {
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
