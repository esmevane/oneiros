pub mod continuity_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct AgentParam {
        agent: AgentName,
    }

    #[derive(Deserialize, JsonSchema)]
    struct SenseParams {
        agent: AgentName,
        content: Content,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "dream",
                description: "Restore an agent's full identity and cognitive context",
                input_schema: schema_for::<AgentParam>,
            },
            ToolDef {
                name: "introspect",
                description: "Look inward — consolidate what matters",
                input_schema: schema_for::<AgentParam>,
            },
            ToolDef {
                name: "reflect",
                description: "Pause on something significant",
                input_schema: schema_for::<AgentParam>,
            },
            ToolDef {
                name: "sense",
                description: "Receive and interpret something from outside",
                input_schema: schema_for::<SenseParams>,
            },
            ToolDef {
                name: "sleep",
                description: "End a session — capture continuity before resting",
                input_schema: schema_for::<AgentParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["dream", "introspect", "reflect", "sense", "sleep"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let no_overrides = DreamOverrides::default();

        let value = match tool_name {
            "dream" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ContinuityService::dream(context, &p.agent, &no_overrides)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "introspect" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ContinuityService::introspect(context, &p.agent, &no_overrides)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "reflect" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ContinuityService::reflect(context, &p.agent, &no_overrides)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "sense" => {
                let p: SenseParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    ContinuityService::sense(context, &p.agent, &p.content, &no_overrides)
                        .await
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "sleep" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ContinuityService::sleep(context, &p.agent, &no_overrides)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
