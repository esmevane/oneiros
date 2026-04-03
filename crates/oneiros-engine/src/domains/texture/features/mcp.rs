use crate::*;

pub struct TextureTools;

impl TextureTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        texture_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        texture_mcp::tool_names()
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_texture",
                description: "Define a quality of thought",
                input_schema: schema_for::<SetTexture>,
            },
            ToolDef {
                name: "get_texture",
                description: "Look up a thought category",
                input_schema: schema_for::<GetTexture>,
            },
            ToolDef {
                name: "list_textures",
                description: "See all the ways thoughts can be textured",
                input_schema: schema_for::<ListTextures>,
            },
            ToolDef {
                name: "remove_texture",
                description: "Remove a thought category",
                input_schema: schema_for::<RemoveTexture>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "set_texture",
            "get_texture",
            "list_textures",
            "remove_texture",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_texture" => TextureService::set(context, &serde_json::from_str(params)?).await,
            "get_texture" => TextureService::get(context, &serde_json::from_str(params)?).await,
            "list_textures" => TextureService::list(context, &serde_json::from_str(params)?).await,
            "remove_texture" => {
                TextureService::remove(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
