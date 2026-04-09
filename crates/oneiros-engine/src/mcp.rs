//! MCP tool server — thin rmcp adapter delegating to per-domain dispatchers.
//!
//! Each domain owns its tool catalog in `features/mcp.rs`. This module
//! collects them into an rmcp ServerHandler, routing tool calls to the
//! appropriate domain dispatcher.
//!
//! Session authentication: if the MCP client sends an `Authorization:
//! Bearer <token>` header during the initialize handshake, the session
//! resolves to the brain associated with that token. Without a token,
//! the session uses the server's default brain.

use std::sync::OnceLock;

use rmcp::model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo, Tool};
use rmcp::{ErrorData, ServerHandler};

use crate::*;

/// Errors that can occur during MCP tool dispatch.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Application error: {0}")]
    App(#[from] Error),

    #[error("Malformed input: {0}")]
    Malformed(#[from] serde_json::Error),

    /// The requested tool name is not handled by this domain.
    #[error("Unknown tool: {0}")]
    UnknownTool(String),

    /// A parameter could not be deserialized or was otherwise invalid.
    #[error("Parameter error: {0}")]
    Parameter(String),

    /// The underlying domain service returned an error.
    #[error("Domain error: {0}")]
    Domain(String),
}

/// Collect all tool definitions from every domain's catalog.
fn all_tools() -> Vec<&'static ToolDef> {
    let sources: &[&[ToolDef]] = &[
        ActorTools.defs(),
        TenantTools.defs(),
        BrainTools.defs(),
        TicketTools.defs(),
        BookmarkTools.defs(),
        LevelTools.defs(),
        TextureTools.defs(),
        SensationTools.defs(),
        NatureTools.defs(),
        PersonaTools.defs(),
        UrgeTools.defs(),
        AgentTools.defs(),
        CognitionTools.defs(),
        MemoryTools.defs(),
        ExperienceTools.defs(),
        ConnectionTools.defs(),
        ContinuityTools.defs(),
        SearchTools.defs(),
        StorageTools.defs(),
        PressureTools.defs(),
    ];

    sources.iter().flat_map(|s| s.iter()).collect()
}

/// Domain dispatch table — routes tool names to domain dispatchers.
///
/// Derives ProjectContext or SystemContext from ServerState as needed.
/// Bookmark tools get ServerState directly for CanonIndex access.
async fn dispatch(
    state: &ServerState,
    config: &Config,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    // Bookmark tools — need ServerState for CanonIndex
    if BookmarkTools.names().contains(&tool_name) {
        return BookmarkTools.dispatch(state, tool_name, params).await;
    }

    // All other tools work through ProjectContext
    let context = state
        .project_context(config.clone())
        .map_err(|e| ToolError::Domain(e.to_string()))?;

    // System domains
    if ActorTools.names().contains(&tool_name) {
        return ActorTools.dispatch(&context, tool_name, params).await;
    }
    if TenantTools.names().contains(&tool_name) {
        return TenantTools.dispatch(&context, tool_name, params).await;
    }
    if BrainTools.names().contains(&tool_name) {
        return BrainTools.dispatch(&context, tool_name, params).await;
    }
    if TicketTools.names().contains(&tool_name) {
        return TicketTools.dispatch(&context, tool_name, params).await;
    }
    // Project domains
    if LevelTools.names().contains(&tool_name) {
        return LevelTools.dispatch(&context, tool_name, params).await;
    }
    if TextureTools.names().contains(&tool_name) {
        return TextureTools.dispatch(&context, tool_name, params).await;
    }
    if SensationTools.names().contains(&tool_name) {
        return SensationTools.dispatch(&context, tool_name, params).await;
    }
    if NatureTools.names().contains(&tool_name) {
        return NatureTools.dispatch(&context, tool_name, params).await;
    }
    if PersonaTools.names().contains(&tool_name) {
        return PersonaTools.dispatch(&context, tool_name, params).await;
    }
    if UrgeTools.names().contains(&tool_name) {
        return UrgeTools.dispatch(&context, tool_name, params).await;
    }
    if AgentTools.names().contains(&tool_name) {
        return AgentTools.dispatch(&context, tool_name, params).await;
    }
    if CognitionTools.names().contains(&tool_name) {
        return CognitionTools.dispatch(&context, tool_name, params).await;
    }
    if MemoryTools.names().contains(&tool_name) {
        return MemoryTools.dispatch(&context, tool_name, params).await;
    }
    if ExperienceTools.names().contains(&tool_name) {
        return ExperienceTools.dispatch(&context, tool_name, params).await;
    }
    if ConnectionTools.names().contains(&tool_name) {
        return ConnectionTools.dispatch(&context, tool_name, params).await;
    }
    if ContinuityTools.names().contains(&tool_name) {
        return ContinuityTools.dispatch(&context, tool_name, params).await;
    }
    if SearchTools.names().contains(&tool_name) {
        return SearchTools.dispatch(&context, tool_name, params).await;
    }
    if StorageTools.names().contains(&tool_name) {
        return StorageTools.dispatch(&context, tool_name, params).await;
    }
    if PressureTools.names().contains(&tool_name) {
        return PressureTools.dispatch(&context, tool_name, params).await;
    }

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

/// Resolve a brain-specific config from a Bearer token.
///
/// Follows the same validation as the HTTP auth layer: decode the
/// self-describing token, verify the ticket exists, and resolve the
/// brain name.
async fn resolve_config_from_token(state: &ServerState, token_str: &str) -> Option<Config> {
    let token = Token::from(token_str).decode().ok()?;

    let system = state.config().system();
    let ticket = TicketRepo::new(&system)
        .get_by_token(token_str)
        .await
        .ok()
        .flatten()?;

    if ticket.actor_id != token.actor_id || ticket.brain_id != token.brain_id {
        return None;
    }

    let mut config = state.config().clone();
    config.brain = ticket.brain_name;
    Some(config)
}

/// MCP tool server wrapping the server state.
///
/// Each MCP session gets its own `EngineToolBox`. During the initialize
/// handshake, if a Bearer token is present, the session resolves to
/// the brain associated with that token. Otherwise it uses the server's
/// default brain config.
#[derive(Clone)]
pub struct EngineToolBox {
    state: ServerState,
    /// Session-resolved config, set during initialize from Bearer token.
    /// Falls back to state.config() if not set.
    session_config: OnceLock<Config>,
}

impl EngineToolBox {
    pub fn new(state: ServerState) -> Self {
        Self {
            state,
            session_config: OnceLock::new(),
        }
    }

    /// The config for this session — resolved from token if authenticated,
    /// otherwise the server default.
    fn config(&self) -> &Config {
        self.session_config.get().unwrap_or(self.state.config())
    }
}

impl ServerHandler for EngineToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new("oneiros-engine", env!("CARGO_PKG_VERSION")),
        )
    }

    async fn initialize(
        &self,
        request: rmcp::model::InitializeRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::InitializeResult, ErrorData> {
        // Extract Bearer token from HTTP headers if present
        if let Some(parts) = context.extensions.get::<axum::http::request::Parts>()
            && let Some(auth) = parts.headers.get("authorization")
            && let Some(token_str) = auth.to_str().ok().and_then(|s| s.strip_prefix("Bearer "))
            && let Some(config) = resolve_config_from_token(&self.state, token_str).await
        {
            let _ = self.session_config.set(config);
        }

        // Delegate to default initialization
        if context.peer.peer_info().is_none() {
            context.peer.set_peer_info(request);
        }

        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, ErrorData> {
        let tools = all_tools()
            .into_iter()
            .map(|t| {
                let mut tool = Tool::default();
                tool.name = t.name.into();
                tool.description = Some(t.description.into());
                tool.input_schema = serde_json::from_value((t.input_schema)())
                    .expect("schema should be a JSON object");
                tool
            })
            .collect();

        Ok(rmcp::model::ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_name = request.name.as_ref();
        let params = serde_json::to_string(&request.arguments.unwrap_or_default())
            .unwrap_or_else(|_| "{}".to_string());

        match dispatch(&self.state, self.config(), tool_name, &params).await {
            Ok(value) => Ok(CallToolResult::success(vec![
                Content::json(value).expect("content"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}
