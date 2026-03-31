use crate::*;

pub struct SearchTools;

impl SearchTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        search_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        search_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        search_mcp::dispatch(context, tool_name, params).await
    }
}

mod search_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[ToolDef {
            name: "search",
            description: "Search across everything in the brain",
            input_schema: schema_for::<SearchQuery>,
        }]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["search"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "search" => SearchService::search(context, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
