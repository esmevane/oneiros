use crate::*;

pub struct AgentTools;

impl AgentTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        agent_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        agent_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourceDef::new(
            "oneiros-mcp://agents",
            "agents",
            "All agents in the current brain",
        )]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        match path {
            "agents" => Some(agent_mcp::read_agents(context).await),
            _ => None,
        }
    }
}

mod agent_mcp {
    use crate::*;

    pub async fn read_agents(context: &ProjectContext) -> Result<String, ToolError> {
        let filters = SearchFilters::default();
        let response = AgentService::list(context, &ListAgents { filters })
            .await
            .map_err(Error::from)?;

        let mut md = String::from("# Agents\n\n");
        match response {
            AgentResponse::Agents(listed) => {
                md.push_str(&format!("{} of {} total\n\n", listed.len(), listed.total));
                md.push_str("| Name | Persona | Description |\n");
                md.push_str("|------|---------|-------------|\n");
                for wrapped in &listed.items {
                    let a = &wrapped.data;
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        a.name, a.persona, a.description
                    ));
                }
            }
            AgentResponse::NoAgents => md.push_str("No agents configured.\n"),
            _ => {}
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateAgent>::def(
                AgentRequestType::CreateAgent,
                "Bring a new agent into the brain",
            ),
            Tool::<GetAgent>::def(AgentRequestType::GetAgent, "Learn about a specific agent"),
            Tool::<ListAgents>::def(AgentRequestType::ListAgents, "See who's here"),
            Tool::<UpdateAgent>::def(AgentRequestType::UpdateAgent, "Reshape an agent's identity"),
            Tool::<RemoveAgent>::def(
                AgentRequestType::RemoveAgent,
                "Remove an agent from the brain",
            ),
        ]
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

        let request_type: AgentRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            AgentRequestType::CreateAgent => {
                let resp = AgentService::create(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    AgentResponse::AgentCreated(name) => {
                        let response = McpResponse::new(format!("Agent created: {name}"))
                            .hint(Hint::follow_up(
                                "dream-agent",
                                "Restore identity and cognitive context",
                            ))
                            .hint(Hint::inspect("oneiros-mcp://agents", "See all agents"));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            AgentRequestType::GetAgent => {
                let resp = AgentService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    AgentResponse::AgentDetails(wrapped) => {
                        let a = &wrapped.data;
                        let body = format!(
                            "**name:** {}\n**persona:** {}\n**description:** {}\n",
                            a.name, a.persona, a.description
                        );
                        Ok(McpResponse::new(body))
                    }
                    AgentResponse::NoAgents => Ok(McpResponse::new("Agent not found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            AgentRequestType::ListAgents => {
                let request: ListAgents = serde_json::from_str(params).unwrap_or_default();
                let resp = AgentService::list(&context, &request)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    AgentResponse::Agents(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            let a = &wrapped.data;
                            body.push_str(&format!(
                                "- **{}** ({}) — {}\n",
                                a.name, a.persona, a.description
                            ));
                        }
                        Ok(McpResponse::new(body))
                    }
                    AgentResponse::NoAgents => Ok(McpResponse::new("No agents configured.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            AgentRequestType::UpdateAgent => {
                let resp = AgentService::update(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    AgentResponse::AgentUpdated(name) => {
                        Ok(McpResponse::new(format!("Agent updated: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            AgentRequestType::RemoveAgent => {
                let resp = AgentService::remove(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    AgentResponse::AgentRemoved(name) => {
                        Ok(McpResponse::new(format!("Agent removed: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
