use crate::*;

pub struct NatureTools;

impl NatureTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        nature_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        nature_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        nature_mcp::dispatch(context, tool_name, params).await
    }
}

mod nature_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_nature",
                description: "Define a kind of relationship between things",
                input_schema: schema_for::<SetNature>,
            },
            ToolDef {
                name: "get_nature",
                description: "Look up a relationship category",
                input_schema: schema_for::<GetNature>,
            },
            ToolDef {
                name: "list_natures",
                description: "See all the kinds of relationships",
                input_schema: schema_for::<ListNatures>,
            },
            ToolDef {
                name: "remove_nature",
                description: "Remove a relationship category",
                input_schema: schema_for::<RemoveNature>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["set_nature", "get_nature", "list_natures", "remove_nature"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_nature" => NatureService::set(context, &serde_json::from_str(params)?).await,
            "get_nature" => NatureService::get(context, &serde_json::from_str(params)?).await,
            "list_natures" => NatureService::list(context, &serde_json::from_str(params)?).await,
            "remove_nature" => NatureService::remove(context, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
