pub mod agent_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct NameParam {
        name: AgentName,
    }

    #[derive(Deserialize, JsonSchema)]
    struct AgentParams {
        name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_agent",
                description: "Bring a new agent into the brain",
                input_schema: schema_for::<AgentParams>,
            },
            ToolDef {
                name: "get_agent",
                description: "Learn about a specific agent",
                input_schema: schema_for::<NameParam>,
            },
            ToolDef {
                name: "list_agents",
                description: "See who's here",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "update_agent",
                description: "Reshape an agent's identity",
                input_schema: schema_for::<AgentParams>,
            },
            ToolDef {
                name: "remove_agent",
                description: "Remove an agent from the brain",
                input_schema: schema_for::<NameParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &[
            "create_agent",
            "get_agent",
            "list_agents",
            "update_agent",
            "remove_agent",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "create_agent" => {
                let p: AgentParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    AgentService::create(context, p.name, p.persona, p.description, p.prompt)
                        .await
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_agent" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = AgentService::get(context, &p.name)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_agents" => {
                let response =
                    AgentService::list(context).map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "update_agent" => {
                let p: AgentParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response =
                    AgentService::update(context, p.name, p.persona, p.description, p.prompt)
                        .await
                        .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_agent" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = AgentService::remove(context, &p.name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
