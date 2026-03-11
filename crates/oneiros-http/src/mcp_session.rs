use std::sync::RwLock;

use axum::http::request::Parts;
use oneiros_mcp::OneirosToolBox;
use oneiros_service::OneirosService;
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        CallToolRequestParams, CallToolResult, InitializeRequestParams, InitializeResult,
        ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, PaginatedRequestParams,
        ReadResourceRequestParams, ReadResourceResult, ResourceUpdatedNotification,
        ResourceUpdatedNotificationParam, ServerInfo, ServerNotification, SubscribeRequestParams,
        UnsubscribeRequestParams,
    },
    service::RequestContext,
};

/// HTTP-transport adapter for MCP sessions.
///
/// Wraps `OneirosToolBox` with `RwLock` because rmcp's `initialize(&self)`
/// is where we resolve Bearer token auth and upgrade from system-only to
/// brain-scoped capability. After initialization the toolbox is stable.
///
/// Async `ServerHandler` methods snapshot the toolbox (cheap — all `Arc`s
/// internally) so the lock guard doesn't cross `.await` boundaries.
pub struct McpSession {
    toolbox: RwLock<OneirosToolBox>,
}

impl McpSession {
    pub fn new(state: OneirosService) -> Self {
        Self {
            toolbox: RwLock::new(OneirosToolBox::system(state)),
        }
    }

    /// Snapshot the current toolbox for use in async contexts.
    fn snapshot(&self) -> OneirosToolBox {
        self.toolbox.read().expect("toolbox lock poisoned").clone()
    }
}

impl ServerHandler for McpSession {
    fn get_info(&self) -> ServerInfo {
        self.snapshot().get_info()
    }

    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        if let Some(parts) = context.extensions.get::<Parts>() {
            tracing::debug!("MCP initialize: found HTTP parts, attempting auth");

            let token = parts
                .headers
                .get("authorization")
                .and_then(|header_value| header_value.to_str().ok())
                .and_then(|header_value| header_value.strip_prefix("Bearer "));

            if let Some(token_str) = token {
                let upgraded = self.snapshot().upgrade(token_str);

                match upgraded {
                    Ok(new_toolbox) => {
                        *self.toolbox.write().expect("toolbox lock poisoned") = new_toolbox;
                        tracing::info!("MCP initialize: brain context resolved");
                    }
                    Err(error) => {
                        tracing::warn!("MCP initialize: auth failed: {error}");
                    }
                }
            } else {
                tracing::debug!("MCP initialize: no Bearer token, staying in system mode");
            }
        } else {
            tracing::debug!("MCP initialize: no HTTP parts in context, staying in system mode");
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
        self.snapshot().list_tools(request, context).await
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let toolbox = self.snapshot();
        let result = toolbox.call_tool(request, context.clone()).await?;

        // Check pressure thresholds after state-mutating tool calls.
        if toolbox.has_subscriptions() {
            for uri in toolbox.check_pressure_thresholds() {
                let notification = ServerNotification::ResourceUpdatedNotification(
                    ResourceUpdatedNotification::new(ResourceUpdatedNotificationParam::new(&uri)),
                );
                if let Err(e) = context.peer.send_notification(notification).await {
                    tracing::warn!("Failed to send pressure notification for {uri}: {e}");
                }
            }
        }

        Ok(result)
    }

    async fn list_resources(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        self.snapshot().list_resources(request, context).await
    }

    async fn list_resource_templates(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
        self.snapshot()
            .list_resource_templates(request, context)
            .await
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        self.snapshot().read_resource(request, context).await
    }

    async fn subscribe(
        &self,
        request: SubscribeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.snapshot().subscribe(request, context).await
    }

    async fn unsubscribe(
        &self,
        request: UnsubscribeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.snapshot().unsubscribe(request, context).await
    }
}
