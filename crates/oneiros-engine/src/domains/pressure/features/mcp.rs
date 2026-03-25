pub mod pressure_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct AgentParam {
        agent: AgentName,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "get_pressure",
                description: "Check pressure for an agent",
                input_schema: schema_for::<AgentParam>,
            },
            ToolDef {
                name: "list_pressures",
                description: "See all pressure readings",
                input_schema: schema_for::<serde_json::Value>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["get_pressure", "list_pressures"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "get_pressure" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = PressureService::get(context, &p.agent)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_pressures" => {
                let response = PressureService::list(context)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
