use crate::*;

pub struct ConnectionTools;

impl ConnectionTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        connection_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        connection_mcp::dispatch(context, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourceTemplateDef::new(
            "oneiros-mcp://agent/{name}/connections",
            "agent-connections",
            "Relationship web for an agent",
        )]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if let Some(rest) = path.strip_prefix("agent/") {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            if parts.len() == 2 && parts[1] == "connections" {
                let agent_name = parts[0];
                return Some(connection_mcp::read_connections(context, agent_name).await);
            }
        }
        None
    }
}

mod connection_mcp {
    use crate::*;

    pub async fn read_connections(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = ConnectionService::list(context, &ListConnections::builder().build())
            .await
            .map_err(Error::from)?;

        let mut md = format!("# Connections — {agent_name}\n\n");
        match response {
            ConnectionResponse::Connections(listed) => {
                md.push_str(&format!("{} of {} total\n\n", listed.len(), listed.total));
                for wrapped in &listed.items {
                    let c = &wrapped.data;
                    md.push_str(&format!(
                        "- **{}** {:?} → {:?}\n",
                        c.nature, c.from_ref, c.to_ref
                    ));
                }
            }
            ConnectionResponse::NoConnections => md.push_str("No connections.\n"),
            _ => {}
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateConnection>::def(
                ConnectionRequestType::CreateConnection,
                "Draw a line between two related things",
            ),
            Tool::<GetConnection>::def(
                ConnectionRequestType::GetConnection,
                "Examine a specific connection",
            ),
            Tool::<ListConnections>::def(
                ConnectionRequestType::ListConnections,
                "See how things connect",
            ),
            Tool::<RemoveConnection>::def(
                ConnectionRequestType::RemoveConnection,
                "Remove a connection",
            ),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let request_type: ConnectionRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            ConnectionRequestType::CreateConnection => {
                let resp = ConnectionService::create(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ConnectionResponse::ConnectionCreated(wrapped) => {
                        let c = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "Connection created: **{}** {:?} → {:?}",
                            c.nature, c.from_ref, c.to_ref
                        )))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            ConnectionRequestType::GetConnection => {
                let resp = ConnectionService::get(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ConnectionResponse::ConnectionDetails(wrapped) => {
                        let c = &wrapped.data;
                        let body = format!(
                            "**nature:** {}\n**from:** {:?}\n**to:** {:?}\n",
                            c.nature, c.from_ref, c.to_ref
                        );
                        Ok(McpResponse::new(body))
                    }
                    ConnectionResponse::NoConnections => {
                        Ok(McpResponse::new("Connection not found."))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            ConnectionRequestType::ListConnections => {
                let resp = ConnectionService::list(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ConnectionResponse::Connections(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            let c = &wrapped.data;
                            body.push_str(&format!(
                                "- **{}** {:?} → {:?}\n",
                                c.nature, c.from_ref, c.to_ref
                            ));
                        }
                        Ok(McpResponse::new(body))
                    }
                    ConnectionResponse::NoConnections => Ok(McpResponse::new("No connections.")),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            ConnectionRequestType::RemoveConnection => {
                let resp = ConnectionService::remove(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ConnectionResponse::ConnectionRemoved(id) => {
                        Ok(McpResponse::new(format!("Connection removed: {id:?}")))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
        }
    }
}
