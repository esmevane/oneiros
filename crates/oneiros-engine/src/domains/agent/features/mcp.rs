use crate::*;

pub struct AgentMcp;

impl AgentMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        agent_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        agent_mcp::dispatch(context, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Agents.resource_def("All agents in the brain")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![
            ResourcePathKind::Agent.into_template("Agent details"),
            ResourcePathKind::AgentCognitions.into_template("An agent's cognitions"),
            ResourcePathKind::AgentMemories.into_template("An agent's memories"),
            ResourcePathKind::AgentExperiences.into_template("An agent's experiences"),
            ResourcePathKind::AgentConnections.into_template("An agent's connections"),
            ResourcePathKind::AgentPressure.into_template("An agent's pressure readings"),
        ]
    }

    pub async fn resource(
        &self,
        context: &ProjectContext,
        request: &AgentRequest,
    ) -> Result<McpResponse, ToolError> {
        agent_mcp::resource(context, request).await
    }

    /// Handle resource paths that cannot be expressed as a typed request
    /// without I/O. Currently only `AgentConnections`, which requires an
    /// agent lookup to resolve a `RefToken` before building a
    /// `ConnectionRequest`.
    pub async fn read_resource_special(
        &self,
        context: &ProjectContext,
        path: &ResourcePath,
    ) -> Result<McpResponse, ToolError> {
        match path {
            ResourcePath::AgentConnections(name) => {
                agent_mcp::read_agent_connections(context, name).await
            }
            _ => Err(ToolError::NotFound(format!(
                "No special resource handler for path: {:?}",
                path.kind()
            ))),
        }
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
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        let request_type: AgentRequestType = tool_name
            .as_str()
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            AgentRequestType::CreateAgent => {
                let response =
                    AgentService::create(context, &serde_json::from_value(params.clone())?)
                        .await
                        .map_err(Error::from)?;
                Ok(AgentView::new(response).mcp())
            }
            AgentRequestType::UpdateAgent => {
                let response =
                    AgentService::update(context, &serde_json::from_value(params.clone())?)
                        .await
                        .map_err(Error::from)?;
                Ok(AgentView::new(response).mcp())
            }
            AgentRequestType::RemoveAgent => {
                let response =
                    AgentService::remove(context, &serde_json::from_value(params.clone())?)
                        .await
                        .map_err(Error::from)?;
                Ok(AgentView::new(response).mcp())
            }
            AgentRequestType::GetAgent | AgentRequestType::ListAgents => {
                Err(ToolError::UnknownTool(tool_name.to_string()))
            }
        }
    }

    pub async fn resource(
        context: &ProjectContext,
        request: &AgentRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            AgentRequest::GetAgent(get) => {
                let response = AgentService::get(context, get).await.map_err(Error::from)?;
                match &response {
                    AgentResponse::NoAgents => {
                        Err(ToolError::NotFound("Agent not found".to_string()))
                    }
                    _ => Ok(AgentView::new(response).mcp()),
                }
            }
            AgentRequest::ListAgents(listing) => {
                let response = AgentService::list(context, listing)
                    .await
                    .map_err(Error::from)?;
                Ok(AgentView::new(response).mcp())
            }
            AgentRequest::CreateAgent(_)
            | AgentRequest::UpdateAgent(_)
            | AgentRequest::RemoveAgent(_) => Err(ToolError::NotAResource(
                "Mutations are tools, not resources".to_string(),
            )),
        }
    }

    pub async fn read_agent_connections(
        context: &ProjectContext,
        name: &AgentName,
    ) -> Result<McpResponse, ToolError> {
        let agent = AgentService::get(context, &GetAgent { name: name.clone() })
            .await
            .map_err(Error::from)?;

        let agent_ref = match agent {
            AgentResponse::AgentDetails(wrapped) => RefToken::from(Ref::agent(wrapped.data.id)),
            _ => return Err(ToolError::NotFound(format!("Agent not found: {name}"))),
        };

        let listing = ListConnections {
            entity: Some(agent_ref),
            filters: SearchFilters::default(),
        };
        let request = ConnectionRequest::ListConnections(listing.clone());
        ConnectionMcp.resource(context, &request).await
    }
}
