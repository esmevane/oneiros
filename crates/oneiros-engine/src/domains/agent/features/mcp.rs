use crate::*;

pub struct AgentTools;

impl AgentTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        agent_mcp::tool_defs()
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

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateAgent>::new(
                AgentRequestType::CreateAgent,
                "Bring a new agent into the brain",
            )
            .def(),
            Tool::<GetAgent>::new(AgentRequestType::GetAgent, "Learn about a specific agent").def(),
            Tool::<ListAgents>::new(AgentRequestType::ListAgents, "See who's here").def(),
            Tool::<UpdateAgent>::new(AgentRequestType::UpdateAgent, "Reshape an agent's identity")
                .def(),
            Tool::<RemoveAgent>::new(
                AgentRequestType::RemoveAgent,
                "Remove an agent from the brain",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: AgentRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            AgentRequestType::CreateAgent => {
                AgentService::create(context, &serde_json::from_str(params)?).await
            }
            AgentRequestType::GetAgent => {
                AgentService::get(context, &serde_json::from_str(params)?).await
            }
            AgentRequestType::ListAgents => {
                let request: ListAgents = serde_json::from_str(params).unwrap_or_default();
                AgentService::list(context, &request).await
            }
            AgentRequestType::UpdateAgent => {
                AgentService::update(context, &serde_json::from_str(params)?).await
            }
            AgentRequestType::RemoveAgent => {
                AgentService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
