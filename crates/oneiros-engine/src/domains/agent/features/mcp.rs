pub mod agent_mcp {
    //! Agent MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct NameParam {
        name: String,
    }

    #[derive(serde::Deserialize, schemars::JsonSchema)]
    struct AgentParams {
        name: String,
        persona: String,
        description: String,
        prompt: String,
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

    pub fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "create_agent" => {
                let p: AgentParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = AgentService::create(
                    context,
                    AgentName::new(&p.name),
                    PersonaName::new(&p.persona),
                    Description::new(&p.description),
                    Prompt::new(&p.prompt),
                )
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_agent" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = AgentService::get(context, &AgentName::new(&p.name))
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
                let response = AgentService::update(
                    context,
                    AgentName::new(&p.name),
                    PersonaName::new(&p.persona),
                    Description::new(&p.description),
                    Prompt::new(&p.prompt),
                )
                .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_agent" => {
                let p: NameParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = AgentService::remove(context, &AgentName::new(&p.name))
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
