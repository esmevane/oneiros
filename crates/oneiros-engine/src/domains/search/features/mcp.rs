use crate::*;

pub struct SearchTools;

impl SearchTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        search_mcp::tool_defs()
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

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SearchQuery>::new(
                SearchRequestType::SearchQuery,
                "Search across everything in the brain",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: SearchRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            SearchRequestType::SearchQuery => {
                SearchService::search(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
