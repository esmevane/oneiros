use crate::*;

pub struct AgentTools;

impl AgentTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        agent_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        agent_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        agent_mcp::dispatch(context, tool_name, params).await
    }
}

mod agent_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_agent",
                description: "Bring a new agent into the brain",
                input_schema: schema_for::<CreateAgent>,
            },
            ToolDef {
                name: "get_agent",
                description: "Learn about a specific agent",
                input_schema: schema_for::<GetAgent>,
            },
            ToolDef {
                name: "list_agents",
                description: "See who's here",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "update_agent",
                description: "Reshape an agent's identity",
                input_schema: schema_for::<UpdateAgent>,
            },
            ToolDef {
                name: "remove_agent",
                description: "Remove an agent from the brain",
                input_schema: schema_for::<RemoveAgent>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
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
            "create_agent" => AgentService::create(context, &serde_json::from_str(params)?).await,
            "get_agent" => AgentService::get(context, &serde_json::from_str(params)?).await,
            "list_agents" => {
                let request: ListAgents = serde_json::from_str(params).unwrap_or_default();
                AgentService::list(context, &request).await
            }
            "update_agent" => AgentService::update(context, &serde_json::from_str(params)?).await,
            "remove_agent" => AgentService::remove(context, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
