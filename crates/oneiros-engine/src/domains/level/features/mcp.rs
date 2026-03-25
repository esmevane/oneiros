pub mod level_mcp {
    //! Level MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct NameParam {
        name: String,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_level",
                description: "Define how long a kind of memory should be kept",
                input_schema: schema_for::<Level>,
            },
            ToolDef {
                name: "get_level",
                description: "Look up a memory retention tier",
                input_schema: schema_for::<NameParam>,
            },
            ToolDef {
                name: "list_levels",
                description: "See all memory retention tiers",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_level",
                description: "Remove a memory retention tier",
                input_schema: schema_for::<NameParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["set_level", "get_level", "list_levels", "remove_level"]
    }

    pub async fn dispatch(
        ctx: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_level" => {
                let level: Level = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = LevelService::set(ctx, level)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_level" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = LevelService::get(ctx, &LevelName::new(p.name))
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_levels" => {
                let response =
                    LevelService::list(ctx).map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_level" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = LevelService::remove(ctx, &LevelName::new(p.name))
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
