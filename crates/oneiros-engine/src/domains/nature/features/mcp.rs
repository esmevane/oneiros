pub mod nature_mcp {
    //! Nature MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct NameParam {
        name: NatureName,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "set_nature",
                description: "Define a kind of relationship between things",
                input_schema: schema_for::<Nature>,
            },
            ToolDef {
                name: "get_nature",
                description: "Look up a relationship category",
                input_schema: schema_for::<NameParam>,
            },
            ToolDef {
                name: "list_natures",
                description: "See all relationship categories",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "remove_nature",
                description: "Remove a relationship category",
                input_schema: schema_for::<NameParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["set_nature", "get_nature", "list_natures", "remove_nature"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "set_nature" => {
                let nature: Nature = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = NatureService::set(context, nature)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_nature" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = NatureService::get(context, &p.name)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_natures" => {
                let response =
                    NatureService::list(context).map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_nature" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = NatureService::remove(context, &p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
