pub mod texture_mcp {
    //! Texture MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct NameParam {
        name: String,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_texture",
                description: "Define a quality of thought",
                input_schema: schema_for::<Texture>,
            },
            ToolDef {
                name: "get_texture",
                description: "Look up a thought category",
                input_schema: schema_for::<NameParam>,
            },
            ToolDef {
                name: "list_textures",
                description: "See all thought categories",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_texture",
                description: "Remove a thought category",
                input_schema: schema_for::<NameParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &[
            "set_texture",
            "get_texture",
            "list_textures",
            "remove_texture",
        ]
    }

    pub fn dispatch(
        ctx: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_texture" => {
                let texture: Texture = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TextureService::set(ctx, texture)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_texture" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TextureService::get(ctx, &TextureName::new(p.name))
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_textures" => {
                let response =
                    TextureService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_texture" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TextureService::remove(ctx, &TextureName::new(p.name))
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
