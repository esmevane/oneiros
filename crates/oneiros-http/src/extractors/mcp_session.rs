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
        match context.extensions.get::<Parts>() {
            Some(parts) => {
                let mut parts = parts.clone();
                tracing::debug!("MCP initialize: found HTTP parts, attempting auth");

                match ActorContext::from_request_parts(&mut parts, self.toolbox.state()).await {
                    Ok(actor_context) => {
                        tracing::info!("MCP initialize: brain context resolved, upgrading");
                        self.toolbox.upgrade(actor_context.into_oneiros_state());
                    }
                    Err(e) => {
                        tracing::warn!("MCP initialize: auth failed: {e}");
                    }
                }
            }
            None => {
                tracing::debug!("MCP initialize: no HTTP parts in context, staying in system mode");
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
