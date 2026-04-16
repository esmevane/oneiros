use crate::*;

pub struct StorageTools;

impl StorageTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        storage_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        storage_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourceDef::new(
            "oneiros-mcp://storage",
            "storage",
            "Stored artifacts",
        )]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn read_resource(
        &self,
        _context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        match path {
            "storage" => Some(Ok(
                "# Storage\n\nUse `list-storage` to browse artifacts.\n".to_string()
            )),
            _ => None,
        }
    }
}

mod storage_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<ListStorage>::def(StorageRequestType::ListStorage, "Browse your archive"),
            Tool::<GetStorage>::def(StorageRequestType::GetStorage, "Retrieve a stored artifact"),
            Tool::<RemoveStorage>::def(
                StorageRequestType::RemoveStorage,
                "Remove a stored artifact",
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

        let request_type: StorageRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            StorageRequestType::ListStorage => {
                let request: ListStorage = serde_json::from_str(params).unwrap_or_default();
                let resp = StorageService::list(&context, &request)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    StorageResponse::Entries(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            let e = &wrapped.data;
                            body.push_str(&format!("- **{}**\n", e.key));
                        }
                        Ok(McpResponse::new(body))
                    }
                    StorageResponse::NoEntries => Ok(McpResponse::new("No stored artifacts.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            StorageRequestType::GetStorage => {
                let resp = StorageService::show(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    StorageResponse::StorageDetails(wrapped) => {
                        let e = &wrapped.data;
                        let body = format!("**key:** {}\n", e.key);
                        Ok(McpResponse::new(body))
                    }
                    StorageResponse::NoEntries => Ok(McpResponse::new("Artifact not found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            StorageRequestType::RemoveStorage => {
                let resp = StorageService::remove(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    StorageResponse::StorageRemoved(key) => {
                        Ok(McpResponse::new(format!("Artifact removed: {key}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            StorageRequestType::UploadStorage => Err(ToolError::UnknownTool(tool_name.to_string())),
        }
    }
}
