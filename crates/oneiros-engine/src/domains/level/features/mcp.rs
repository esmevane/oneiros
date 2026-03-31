use crate::*;

pub struct LevelTools;

impl LevelTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        level_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        level_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        level_mcp::dispatch(context, tool_name, params).await
    }
}

mod level_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_level",
                description: "Define how long a kind of memory should be kept",
                input_schema: schema_for::<SetLevel>,
            },
            ToolDef {
                name: "get_level",
                description: "Look up a memory retention tier",
                input_schema: schema_for::<GetLevel>,
            },
            ToolDef {
                name: "list_levels",
                description: "See all memory retention tiers",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_level",
                description: "Remove a memory retention tier",
                input_schema: schema_for::<RemoveLevel>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["set_level", "get_level", "list_levels", "remove_level"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_level" => LevelService::set(context, &serde_json::from_str(params)?).await,
            "get_level" => LevelService::get(context, &serde_json::from_str(params)?).await,
            "list_levels" => LevelService::list(context).await,
            "remove_level" => LevelService::remove(context, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
