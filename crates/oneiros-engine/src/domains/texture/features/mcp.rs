use crate::*;

pub struct TextureTools;

impl TextureTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        texture_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        texture_mcp::dispatch(context, tool_name, params).await
    }
}

mod texture_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetTexture>::new(
                TextureRequestType::SetTexture,
                "Define a quality of thought",
            )
            .def(),
            Tool::<GetTexture>::new(TextureRequestType::GetTexture, "Look up a thought category")
                .def(),
            Tool::<ListTextures>::new(
                TextureRequestType::ListTextures,
                "See all the ways thoughts can be textured",
            )
            .def(),
            Tool::<RemoveTexture>::new(
                TextureRequestType::RemoveTexture,
                "Remove a thought category",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: TextureRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            TextureRequestType::SetTexture => {
                TextureService::set(context, &serde_json::from_str(params)?).await
            }
            TextureRequestType::GetTexture => {
                TextureService::get(context, &serde_json::from_str(params)?).await
            }
            TextureRequestType::ListTextures => {
                TextureService::list(context, &serde_json::from_str(params)?).await
            }
            TextureRequestType::RemoveTexture => {
                TextureService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
