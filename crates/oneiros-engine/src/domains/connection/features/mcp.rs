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
    ) -> Result<serde_json::Value, ToolError> {
        connection_mcp::dispatch(context, tool_name, params).await
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
            Tool::<GetConnection>::new(
                ConnectionRequestType::GetConnection,
                "Examine a specific connection",
            )
            .def(),
            Tool::<ListConnections>::new(
                ConnectionRequestType::ListConnections,
                "See how things connect",
            )
            .def(),
            Tool::<RemoveConnection>::new(
                ConnectionRequestType::RemoveConnection,
                "Remove a connection",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: ConnectionRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            ConnectionRequestType::CreateConnection => {
                ConnectionService::create(context, &serde_json::from_str(params)?).await
            }
            ConnectionRequestType::GetConnection => {
                ConnectionService::get(context, &serde_json::from_str(params)?).await
            }
            ConnectionRequestType::ListConnections => {
                ConnectionService::list(context, &serde_json::from_str(params)?).await
            }
            ConnectionRequestType::RemoveConnection => {
                ConnectionService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
