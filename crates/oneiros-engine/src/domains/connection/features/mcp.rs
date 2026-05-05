use crate::*;

pub struct ConnectionMcp;

impl ConnectionMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        connection_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        connection_mcp::dispatch(context, mailbox, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourcePathKind::Connection.into_template("A specific connection")]
    }

    pub async fn resource(
        &self,
        context: &ProjectLog,
        request: &ConnectionRequest,
    ) -> Result<McpResponse, ToolError> {
        connection_mcp::resource(context, request).await
    }
}

mod connection_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateConnection>::new(
                ConnectionRequestType::CreateConnection,
                "Draw a line between two related things",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        let request_type: ConnectionRequestType = tool_name
            .as_str()
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            ConnectionRequestType::CreateConnection => {
                let creation: CreateConnection = serde_json::from_value(params.clone())?;
                let request = ConnectionRequest::CreateConnection(creation.clone());
                let scope = context.scope().map_err(Error::from)?;
                let response = ConnectionService::create(scope, mailbox, &creation)
                    .await
                    .map_err(Error::from)?;
                Ok(ConnectionView::new(response, &request).mcp())
            }
            ConnectionRequestType::GetConnection
            | ConnectionRequestType::ListConnections
            | ConnectionRequestType::RemoveConnection => {
                Err(ToolError::UnknownTool(tool_name.to_string()))
            }
        }
    }

    pub async fn resource(
        context: &ProjectLog,
        request: &ConnectionRequest,
    ) -> Result<McpResponse, ToolError> {
        let scope = context.scope().map_err(Error::from)?;
        let response = match request {
            ConnectionRequest::GetConnection(get) => ConnectionService::get(scope, get)
                .await
                .map_err(Error::from)?,
            ConnectionRequest::ListConnections(listing) => ConnectionService::list(scope, listing)
                .await
                .map_err(Error::from)?,
            ConnectionRequest::CreateConnection(_) | ConnectionRequest::RemoveConnection(_) => {
                return Err(ToolError::NotAResource(
                    "Mutations are tools, not resources".to_string(),
                ));
            }
        };

        match &response {
            ConnectionResponse::NoConnections => {
                Err(ToolError::NotFound("Connection not found".to_string()))
            }
            _ => Ok(ConnectionView::new(response, request).mcp()),
        }
    }
}
