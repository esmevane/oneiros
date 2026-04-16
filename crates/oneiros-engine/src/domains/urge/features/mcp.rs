use crate::*;

pub struct UrgeTools;

impl UrgeTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        urge_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        urge_mcp::dispatch(state, config, tool_name, params).await
    }
}

mod urge_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetUrge>::def(UrgeRequestType::SetUrge, "Define a cognitive drive"),
            Tool::<GetUrge>::def(UrgeRequestType::GetUrge, "Look up a cognitive drive"),
            Tool::<ListUrges>::def(UrgeRequestType::ListUrges, "See all cognitive drives"),
            Tool::<RemoveUrge>::def(UrgeRequestType::RemoveUrge, "Remove a cognitive drive"),
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

        let request_type: UrgeRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            UrgeRequestType::SetUrge => {
                let resp = UrgeService::set(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    UrgeResponse::UrgeSet(name) => {
                        Ok(McpResponse::new(format!("Urge set: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            UrgeRequestType::GetUrge => {
                let resp = UrgeService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    UrgeResponse::UrgeDetails(urge) => Ok(McpResponse::new(format!(
                        "**name:** {}\n**description:** {}\n",
                        urge.name, urge.description
                    ))),
                    UrgeResponse::NoUrges => Ok(McpResponse::new("Urge not found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            UrgeRequestType::ListUrges => {
                let resp = UrgeService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    UrgeResponse::Urges(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for urge in &listed.items {
                            body.push_str(&format!("- {}\n", urge.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    UrgeResponse::NoUrges => Ok(McpResponse::new("No urges.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            UrgeRequestType::RemoveUrge => {
                let resp = UrgeService::remove(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    UrgeResponse::UrgeRemoved(name) => {
                        Ok(McpResponse::new(format!("Urge removed: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
