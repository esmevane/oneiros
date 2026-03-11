use axum::{extract::FromRequestParts, http::request::Parts};
use oneiros_mcp::OneirosToolBox;
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

use crate::*;

/// Wraps `OneirosToolBox` to override the MCP `initialize` lifecycle hook
/// with brain resolution from Bearer token auth.
///
/// Uses `ActorContext::from_request_parts` — the same auth path as all
/// REST handlers — then upgrades the toolbox to full brain capability.
///
/// Also handles pressure threshold notifications after tool calls:
/// when subscriptions are active and any pressure exceeds 80%,
/// sends `ResourceUpdatedNotification` to the client.
pub struct McpSession {
    toolbox: OneirosToolBox,
}

impl McpSession {
    pub fn new(state: Arc<ServiceState>) -> Self {
        Self {
            toolbox: OneirosToolBox::system(state),
        }
    }

    /// After a state-mutating tool call, check if any subscribed pressure
    /// resources have crossed the notification threshold and notify.
    async fn check_and_notify(&self, context: &RequestContext<RoleServer>) {
        if !self.toolbox.has_subscriptions() {
            return;
        }

        let triggered = self.toolbox.check_pressure_thresholds();
        for uri in triggered {
            let notification = ServerNotification::ResourceUpdatedNotification(
                ResourceUpdatedNotification::new(ResourceUpdatedNotificationParam::new(&uri)),
            );
            if let Err(e) = context.peer.send_notification(notification).await {
                tracing::warn!("Failed to send pressure notification for {uri}: {e}");
            }
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
        let result = self.toolbox.call_tool(request, context.clone()).await?;
        self.check_and_notify(&context).await;
        Ok(result)
    }

    async fn list_resources(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        self.toolbox.list_resources(request, context).await
    }

    async fn list_resource_templates(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
        self.toolbox.list_resource_templates(request, context).await
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        self.toolbox.read_resource(request, context).await
    }

    async fn subscribe(
        &self,
        request: SubscribeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.toolbox.subscribe(request, context).await
    }

    async fn unsubscribe(
        &self,
        request: UnsubscribeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.toolbox.unsubscribe(request, context).await
    }
}
