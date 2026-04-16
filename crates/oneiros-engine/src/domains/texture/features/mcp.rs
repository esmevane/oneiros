use crate::*;

pub struct TextureTools;

impl TextureTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        texture_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        texture_mcp::dispatch(state, config, tool_name, params).await
    }
}

mod texture_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetTexture>::def(
                TextureRequestType::SetTexture,
                "Define a quality of thought",
            ),
            Tool::<GetTexture>::def(TextureRequestType::GetTexture, "Look up a thought category"),
            Tool::<ListTextures>::def(
                TextureRequestType::ListTextures,
                "See all the ways thoughts can be textured",
            ),
            Tool::<RemoveTexture>::def(
                TextureRequestType::RemoveTexture,
                "Remove a thought category",
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

        let request_type: TextureRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            TextureRequestType::SetTexture => {
                let resp = TextureService::set(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TextureResponse::TextureSet(name) => {
                        Ok(McpResponse::new(format!("Texture set: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            TextureRequestType::GetTexture => {
                let resp = TextureService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TextureResponse::TextureDetails(wrapped) => {
                        let t = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "**name:** {}\n**description:** {}\n",
                            t.name, t.description
                        )))
                    }
                    TextureResponse::NoTextures => Ok(McpResponse::new("Texture not found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            TextureRequestType::ListTextures => {
                let resp = TextureService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TextureResponse::Textures(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    TextureResponse::NoTextures => Ok(McpResponse::new("No textures.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            TextureRequestType::RemoveTexture => {
                let resp = TextureService::remove(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TextureResponse::TextureRemoved(name) => {
                        Ok(McpResponse::new(format!("Texture removed: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
