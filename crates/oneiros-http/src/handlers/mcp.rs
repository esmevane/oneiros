use axum::Router;
use oneiros_service::ServiceState;
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, tower::StreamableHttpService,
};
use std::sync::Arc;

use crate::*;

/// Create the MCP streamable HTTP transport router.
///
/// Each MCP session gets its own `OneirosToolBox`, starting in system-only
/// mode. During the MCP `initialize` handshake, the Bearer token from the
/// HTTP Authorization header is used to resolve the brain and upgrade the
/// toolbox to full capability via `ActorContext`.
pub(crate) fn router<S: Clone + Send + Sync + 'static>(state: Arc<ServiceState>) -> Router<S> {
    let mcp_service = StreamableHttpService::new(
        move || Ok(McpSession::new(state.clone())),
        Arc::new(LocalSessionManager::default()),
        Default::default(),
    );

    Router::new().route_service("/", mcp_service)
}
