use crate::*;

pub struct SearchTools;

impl SearchTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        search_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        search_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourceTemplateDef::new(
            "oneiros-mcp://record/{ref}",
            "record",
            "Any record by its ref token",
        )]
    }

    pub async fn read_resource(
        &self,
        _context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if path.starts_with("record/") {
            return Some(Ok(
                "# Record lookup\n\nRef token resolution is not yet implemented via MCP resources.\n\
                 Use `search-query` to find records by content.\n"
                    .to_string(),
            ));
        }
        None
    }
}

mod search_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![Tool::<SearchQuery>::def(
            SearchRequestType::SearchQuery,
            "Search across everything in the brain",
        )]
    }

    pub async fn dispatch(
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let context = state
            .project_context(config.clone())
            .map_err(|e| ToolError::Domain(e.to_string()))?;

        let request_type: SearchRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            SearchRequestType::SearchQuery => {
                let resp = SearchService::search(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    SearchResponse::Results(results) => {
                        let mut body = format!(
                            "# Search: \"{}\"\n\n{} result(s)\n\n",
                            results.query,
                            results.results.len()
                        );
                        for expr in &results.results {
                            body.push_str(&format!("- **{}** {}\n", expr.kind, expr.content));
                        }
                        Ok(McpResponse::new(body))
                    }
                }
            }
        }
    }
}
