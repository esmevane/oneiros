use crate::*;

pub struct PeerTools;

impl PeerTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        peer_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectLog,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        peer_mcp::dispatch(context, tool_name, params).await
    }
}

mod peer_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddPeer>::new(
                PeerRequestType::AddPeer,
                "Add a known peer by supplying its base64url-encoded address",
            )
            .def(),
            Tool::<GetPeer>::new(PeerRequestType::GetPeer, "Look up a specific peer by ID").def(),
            Tool::<ListPeers>::new(PeerRequestType::ListPeers, "List all known peers").def(),
            Tool::<RemovePeer>::new(PeerRequestType::RemovePeer, "Forget a known peer").def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectLog,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: PeerRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let scope = ComposeScope::new(context.config.clone())
            .host()
            .map_err(Error::from)?;
        let system = scope.host_log();

        let value = match request_type {
            PeerRequestType::AddPeer => {
                PeerService::add(&system, &serde_json::from_str(params)?).await
            }
            PeerRequestType::GetPeer => {
                PeerService::get(&system, &serde_json::from_str(params)?).await
            }
            PeerRequestType::ListPeers => {
                PeerService::list(&system, &serde_json::from_str(params)?).await
            }
            PeerRequestType::RemovePeer => {
                PeerService::remove(&system, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
