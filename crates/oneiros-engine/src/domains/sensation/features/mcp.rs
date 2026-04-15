use crate::*;

pub struct SensationTools;

impl SensationTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        sensation_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        sensation_mcp::dispatch(context, tool_name, params).await
    }
}

mod sensation_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetSensation>::def(
                SensationRequestType::SetSensation,
                "Define a quality of connection between thoughts",
            ),
            Tool::<GetSensation>::def(
                SensationRequestType::GetSensation,
                "Look up an experience category",
            ),
            Tool::<ListSensations>::def(
                SensationRequestType::ListSensations,
                "See all the ways experiences can feel",
            ),
            Tool::<RemoveSensation>::def(
                SensationRequestType::RemoveSensation,
                "Remove an experience category",
            ),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let request_type: SensationRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            SensationRequestType::SetSensation => {
                let resp = SensationService::set(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    SensationResponse::SensationSet(name) => {
                        Ok(McpResponse::new(format!("Sensation set: {name}")))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            SensationRequestType::GetSensation => {
                let resp = SensationService::get(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    SensationResponse::SensationDetails(wrapped) => {
                        let s = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "**name:** {}\n**description:** {}\n",
                            s.name, s.description
                        )))
                    }
                    SensationResponse::NoSensations => Ok(McpResponse::new("Sensation not found.")),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            SensationRequestType::ListSensations => {
                let resp = SensationService::list(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    SensationResponse::Sensations(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    SensationResponse::NoSensations => Ok(McpResponse::new("No sensations.")),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            SensationRequestType::RemoveSensation => {
                let resp = SensationService::remove(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    SensationResponse::SensationRemoved(name) => {
                        Ok(McpResponse::new(format!("Sensation removed: {name}")))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
        }
    }
}
