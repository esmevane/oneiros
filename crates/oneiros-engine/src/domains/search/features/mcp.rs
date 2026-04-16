use crate::*;

pub struct SearchMcp;

impl SearchMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        search_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
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
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        let request_type: SearchRequestType = tool_name
            .as_str()
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            SearchRequestType::SearchQuery => {
                let query: SearchQuery = serde_json::from_value(params.clone())?;
                let response = SearchService::search(context, &query)
                    .await
                    .map_err(Error::from)?;
                Ok(SearchView::new(response).mcp())
            }
        }
    }
}
