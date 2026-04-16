use crate::*;

pub struct MemoryTools;

impl MemoryTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        memory_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        memory_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourceTemplateDef::new(
            "oneiros-mcp://agent/{name}/memories",
            "agent-memories",
            "Consolidated knowledge for an agent",
        )]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if let Some(rest) = path.strip_prefix("agent/") {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            if parts.len() == 2 && parts[1] == "memories" {
                let agent_name = parts[0];
                return Some(memory_mcp::read_memories(context, agent_name).await);
            }
        }
        None
    }
}

mod memory_mcp {
    use crate::*;

    pub async fn read_memories(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = MemoryService::list(
            context,
            &ListMemories::builder()
                .agent(AgentName::new(agent_name))
                .build(),
        )
        .await
        .map_err(Error::from)?;

        let mut md = format!("# Memories — {agent_name}\n\n");
        match response {
            MemoryResponse::Memories(listed) => {
                md.push_str(&format!("{} of {} total\n\n", listed.len(), listed.total));
                for wrapped in &listed.items {
                    let m = &wrapped.data;
                    md.push_str(&format!("- **{}** {}\n", m.level, m.content));
                }
            }
            MemoryResponse::NoMemories => md.push_str("No memories.\n"),
            _ => {}
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddMemory>::def(
                MemoryRequestType::AddMemory,
                "Consolidate something you've learned",
            ),
            Tool::<GetMemory>::def(MemoryRequestType::GetMemory, "Revisit a specific memory"),
            Tool::<ListMemories>::def(MemoryRequestType::ListMemories, "Review what you know"),
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

        let request_type: MemoryRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            MemoryRequestType::AddMemory => {
                let resp = MemoryService::add(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    MemoryResponse::MemoryAdded(wrapped) => {
                        let ref_token = wrapped.meta().ref_token();
                        let mut response = McpResponse::new(format!(
                            "Memory consolidated: {}",
                            wrapped.data.content
                        ));
                        if let Some(rt) = ref_token {
                            response = response.hint(Hint::suggest(
                                format!("create-connection {rt} <target>"),
                                "Link to something related",
                            ));
                        }
                        response =
                            response.hint(Hint::inspect("search-query", "Find related entities"));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            MemoryRequestType::GetMemory => {
                let resp = MemoryService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    MemoryResponse::MemoryDetails(wrapped) => {
                        let m = &wrapped.data;
                        let body = format!("**level:** {}\n**content:** {}\n", m.level, m.content);
                        Ok(McpResponse::new(body))
                    }
                    MemoryResponse::NoMemories => Ok(McpResponse::new("No memory found.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            MemoryRequestType::ListMemories => {
                let resp = MemoryService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    MemoryResponse::Memories(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            let m = &wrapped.data;
                            body.push_str(&format!("- **{}** {}\n", m.level, m.content));
                        }
                        Ok(McpResponse::new(body))
                    }
                    MemoryResponse::NoMemories => Ok(McpResponse::new("No memories.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
