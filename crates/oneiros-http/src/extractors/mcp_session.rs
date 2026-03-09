use axum::{extract::FromRequestParts, http::request::Parts};
use oneiros_mcp::OneirosToolBox;
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        CallToolRequestParams, CallToolResult, InitializeRequestParams, InitializeResult,
        ListToolsResult, PaginatedRequestParams, ServerInfo,
    },
    service::RequestContext,
};

use crate::*;

/// Wraps `OneirosToolBox` to override the MCP `initialize` lifecycle hook
/// with brain resolution from Bearer token auth.
///
/// Uses `ActorContext::from_request_parts` — the same auth path as all
/// REST handlers — then upgrades the toolbox to full brain capability.
pub struct McpSession {
    toolbox: OneirosToolBox,
}

impl McpSession {
    pub fn new(state: Arc<ServiceState>) -> Self {
        Self {
            toolbox: OneirosToolBox::system(state),
        }
    }
}

impl ServerHandler for McpSession {
    fn get_info(&self) -> ServerInfo {
        self.toolbox.get_info()
    }

    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        if let Some(parts) = context.extensions.get::<Parts>() {
            let mut parts = parts.clone();

            if let Ok(context) =
                ActorContext::from_request_parts(&mut parts, self.toolbox.state()).await
            {
                self.toolbox.upgrade(context.into_oneiros_state());
            }
        }

        if context.peer.peer_info().is_none() {
            context.peer.set_peer_info(request);
        }

        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        self.toolbox.list_tools(request, context).await
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        self.toolbox.call_tool(request, context).await
    }
}
