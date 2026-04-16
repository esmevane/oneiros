use crate::*;

pub struct NatureTools;

impl NatureTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        nature_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        nature_mcp::dispatch(state, config, tool_name, params).await
    }
}

mod nature_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetNature>::def(
                NatureRequestType::SetNature,
                "Define a kind of relationship between things",
            ),
            Tool::<GetNature>::def(
                NatureRequestType::GetNature,
                "Look up a relationship category",
            ),
            Tool::<ListNatures>::def(
                NatureRequestType::ListNatures,
                "See all the kinds of relationships",
            ),
            Tool::<RemoveNature>::def(
                NatureRequestType::RemoveNature,
                "Remove a relationship category",
            ),
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

        let request_type: NatureRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            NatureRequestType::SetNature => {
                let resp = NatureService::set(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    NatureResponse::NatureSet(name) => {
                        Ok(McpResponse::new(format!("Nature set: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            NatureRequestType::GetNature => {
                let resp = NatureService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    NatureResponse::NatureDetails(wrapped) => {
                        let n = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "**name:** {}\n**description:** {}\n",
                            n.name, n.description
                        )))
                    }
                    NatureResponse::NoNatures => Ok(McpResponse::new("Nature not found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            NatureRequestType::ListNatures => {
                let resp = NatureService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    NatureResponse::Natures(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    NatureResponse::NoNatures => Ok(McpResponse::new("No natures.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            NatureRequestType::RemoveNature => {
                let resp = NatureService::remove(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    NatureResponse::NatureRemoved(name) => {
                        Ok(McpResponse::new(format!("Nature removed: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
