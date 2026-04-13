use crate::*;

pub(crate) struct PeerTools;

impl PeerTools {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        peer_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        peer_mcp::dispatch(context, tool_name, params).await
    }
}

mod peer_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
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

    pub(crate) async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: PeerRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

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
