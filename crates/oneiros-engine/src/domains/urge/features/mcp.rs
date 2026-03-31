use crate::*;

pub struct UrgeTools;

impl UrgeTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        urge_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        urge_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        urge_mcp::dispatch(context, tool_name, params).await
    }
}

mod urge_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_urge",
                description: "Define a cognitive drive",
                input_schema: schema_for::<SetUrge>,
            },
            ToolDef {
                name: "get_urge",
                description: "Look up a cognitive drive",
                input_schema: schema_for::<GetUrge>,
            },
            ToolDef {
                name: "list_urges",
                description: "See all cognitive drives",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_urge",
                description: "Remove a cognitive drive",
                input_schema: schema_for::<RemoveUrge>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["set_urge", "get_urge", "list_urges", "remove_urge"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_urge" => UrgeService::set(context, &serde_json::from_str(params)?).await,
            "get_urge" => UrgeService::get(context, &serde_json::from_str(params)?).await,
            "list_urges" => UrgeService::list(context).await,
            "remove_urge" => UrgeService::remove(context, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
