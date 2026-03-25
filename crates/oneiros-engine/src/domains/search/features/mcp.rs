pub mod search_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct SearchParams {
        query: String,
        agent: Option<AgentName>,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[ToolDef {
            name: "search",
            description: "Search across everything in the brain",
            input_schema: schema_for::<SearchParams>,
        }]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["search"]
    }

    pub fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "search" => {
                let p: SearchParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = SearchService::search(context, &p.query, p.agent.as_ref())
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
