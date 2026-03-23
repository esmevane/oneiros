//! Continuity MCP driving adapter — translates tool calls into domain service calls.
pub mod continuity_mcp {

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct AgentParam {
        agent: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct SenseParams {
        agent: String,
        content: String,
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

    pub fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let no_overrides = DreamOverrides::default();

        let value = match tool_name {
            "dream" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    ContinuityService::dream(context, &AgentName::new(&p.agent), &no_overrides)
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "introspect" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = ContinuityService::introspect(
                    context,
                    &AgentName::new(&p.agent),
                    &no_overrides,
                )
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "reflect" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    ContinuityService::reflect(context, &AgentName::new(&p.agent), &no_overrides)
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "sense" => {
                let p: SenseParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let content = Content::new(&p.content);
                let response = ContinuityService::sense(
                    context,
                    &AgentName::new(&p.agent),
                    &content,
                    &no_overrides,
                )
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "sleep" => {
                let p: AgentParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    ContinuityService::sleep(context, &AgentName::new(&p.agent), &no_overrides)
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
