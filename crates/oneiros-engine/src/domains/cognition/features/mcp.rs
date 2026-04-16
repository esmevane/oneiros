use crate::*;

pub struct CognitionTools;

impl CognitionTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        cognition_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        cognition_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourceTemplateDef::new(
            "oneiros-mcp://agent/{name}/cognitions",
            "agent-cognitions",
            "Thought stream for an agent",
        )]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if let Some(rest) = path.strip_prefix("agent/") {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            if parts.len() == 2 && parts[1] == "cognitions" {
                let agent_name = parts[0];
                return Some(cognition_mcp::read_cognitions(context, agent_name).await);
            }
        }
        None
    }
}

mod cognition_mcp {
    //! Cognition MCP driving adapter — translates tool calls into domain service calls.

    use crate::*;

    pub async fn read_cognitions(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = CognitionService::list(
            context,
            &ListCognitions {
                agent: Some(AgentName::new(agent_name)),
                texture: None,
                filters: SearchFilters::default(),
            },
        )
        .await
        .map_err(Error::from)?;

        let mut md = format!("# Cognitions — {agent_name}\n\n");
        match response {
            CognitionResponse::Cognitions(listed) => {
                md.push_str(&format!("{} of {} total\n\n", listed.len(), listed.total));
                for wrapped in &listed.items {
                    let c = &wrapped.data;
                    md.push_str(&format!("- **{}** {}\n", c.texture, c.content));
                }
            }
            CognitionResponse::NoCognitions => md.push_str("No cognitions.\n"),
            _ => {}
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddCognition>::def(CognitionRequestType::AddCognition, "Record a thought"),
            Tool::<GetCognition>::def(
                CognitionRequestType::GetCognition,
                "Revisit a specific thought",
            ),
            Tool::<ListCognitions>::def(
                CognitionRequestType::ListCognitions,
                "Review a stream of thoughts",
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

        let request_type: CognitionRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            CognitionRequestType::AddCognition => {
                let resp = CognitionService::add(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    CognitionResponse::CognitionAdded(wrapped) => {
                        let ref_token = wrapped.meta().ref_token();
                        let mut response = McpResponse::new(format!(
                            "Cognition recorded: {}",
                            wrapped.data.content
                        ));
                        if let Some(rt) = ref_token {
                            response = response.hint(Hint::suggest(
                                format!("create-connection {rt} <target>"),
                                "Link to something related",
                            ));
                        }
                        response = response
                            .hint(Hint::suggest(
                                "reflect-agent",
                                "Pause on something significant",
                            ))
                            .hint(Hint::inspect(
                                "oneiros-mcp://agent/{name}/cognitions",
                                "Browse the thought stream",
                            ));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            CognitionRequestType::GetCognition => {
                let resp = CognitionService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    CognitionResponse::CognitionDetails(wrapped) => {
                        let c = &wrapped.data;
                        let body =
                            format!("**texture:** {}\n**content:** {}\n", c.texture, c.content);
                        Ok(McpResponse::new(body))
                    }
                    CognitionResponse::NoCognitions => Ok(McpResponse::new("No cognition found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            CognitionRequestType::ListCognitions => {
                let resp = CognitionService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    CognitionResponse::Cognitions(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            let c = &wrapped.data;
                            body.push_str(&format!("- **{}** {}\n", c.texture, c.content));
                        }
                        Ok(McpResponse::new(body)
                            .hint(Hint::suggest("add-cognition", "Record a new thought")))
                    }
                    CognitionResponse::NoCognitions => Ok(McpResponse::new("No cognitions.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
