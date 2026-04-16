use crate::*;

pub struct LevelTools;

impl LevelTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        level_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        level_mcp::dispatch(state, config, tool_name, params).await
    }
}

mod level_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetLevel>::def(
                LevelRequestType::SetLevel,
                "Define how long a kind of memory should be kept",
            ),
            Tool::<GetLevel>::def(
                LevelRequestType::GetLevel,
                "Look up a memory retention tier",
            ),
            Tool::<ListLevels>::def(
                LevelRequestType::ListLevels,
                "See all memory retention tiers",
            ),
            Tool::<RemoveLevel>::def(
                LevelRequestType::RemoveLevel,
                "Remove a memory retention tier",
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

        let request_type: LevelRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            LevelRequestType::SetLevel => {
                let resp = LevelService::set(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    LevelResponse::LevelSet(name) => {
                        Ok(McpResponse::new(format!("Level set: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            LevelRequestType::GetLevel => {
                let resp = LevelService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    LevelResponse::LevelDetails(wrapped) => {
                        let l = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "**name:** {}\n**description:** {}\n",
                            l.name, l.description
                        )))
                    }
                    LevelResponse::NoLevels => Ok(McpResponse::new("Level not found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            LevelRequestType::ListLevels => {
                let resp = LevelService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    LevelResponse::Levels(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    LevelResponse::NoLevels => Ok(McpResponse::new("No levels.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            LevelRequestType::RemoveLevel => {
                let resp = LevelService::remove(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    LevelResponse::LevelRemoved(name) => {
                        Ok(McpResponse::new(format!("Level removed: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
