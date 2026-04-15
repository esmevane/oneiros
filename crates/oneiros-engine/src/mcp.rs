//! MCP server — the agent interface for oneiros.
//!
//! Exposes tools, resources, and prompts through the Model Context
//! Protocol. Tools are scoped via toolsets — the agent activates a
//! cognitive mode to load the relevant tool surface.
//!
//! Session authentication: if the MCP client sends an `Authorization:
//! Bearer <token>` header during the initialize handshake, the session
//! resolves to the brain associated with that token. Without a token,
//! the session uses the server's default brain.

use rmcp::model::{
    CallToolResult, Content, Implementation, PromptMessage, PromptMessageRole, ResourceContents,
    ServerCapabilities, ServerInfo, Tool as RmcpTool,
};
use rmcp::{ErrorData, ServerHandler};
use std::sync::{Mutex, OnceLock};

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

/// Read a resource by URI, returning authored markdown.
///
/// Delegates to each domain's `read_resource` method — first `Some` wins.
/// Bookmark uses `read_resource_with_state` because it needs `ServerState`.
/// Vocabulary is cross-domain and handled here.
async fn read_resource(
    state: &ServerState,
    config: &Config,
    uri: &str,
) -> Result<String, ToolError> {
    let context = state
        .project_context(config.clone())
        .map_err(|e| ToolError::Domain(e.to_string()))?;

    let path = uri
        .strip_prefix("oneiros-mcp://")
        .ok_or_else(|| ToolError::Parameter(format!("invalid resource URI: {uri}")))?;

    // Domain-owned resources — first Some wins
    if let Some(result) = AgentTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = CognitionTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = MemoryTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = ExperienceTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = ConnectionTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = ContinuityTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = PressureTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = SearchTools.read_resource(&context, path).await {
        return result;
    }
    if let Some(result) = StorageTools.read_resource(&context, path).await {
        return result;
    }

    // Bookmark needs ServerState
    if let Some(result) = BookmarkTools
        .read_resource_with_state(state, config, path)
        .await
    {
        return result;
    }

    // Cross-domain: vocabulary is a view across all vocabulary domains
    if path == "vocabulary" {
        let mut md = String::from("# Vocabulary\n\n");
        for kind in &[
            "levels",
            "textures",
            "sensations",
            "natures",
            "personas",
            "urges",
        ] {
            md.push_str(&format!("## {kind}\n\n"));
        }
        return Ok(md);
    }

    Err(ToolError::Parameter(format!("unknown resource: {uri}")))
}

// ── Domain dispatch ────────────────────────────────────────────

/// Domain dispatch table — routes tool names to domain dispatchers.
async fn dispatch(
    state: &ServerState,
    config: &Config,
    tool_name: &str,
    params: &str,
) -> Result<McpResponse, ToolError> {
    let context = state
        .project_context(config.clone())
        .map_err(|e| ToolError::Domain(e.to_string()))?;

    // Bookmark tools — also need ServerState for CanonIndex and Bridge
    if tool_name.parse::<BookmarkRequestType>().is_ok() {
        return BookmarkTools
            .dispatch(&context, state, tool_name, params)
            .await;
    }

    // System domains
    if tool_name.parse::<ActorRequestType>().is_ok() {
        return ActorTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<TenantRequestType>().is_ok() {
        return TenantTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<BrainRequestType>().is_ok() {
        return BrainTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<TicketRequestType>().is_ok() {
        return TicketTools.dispatch(&context, tool_name, params).await;
    }
    // Project domains
    if tool_name.parse::<AgentRequestType>().is_ok() {
        return AgentTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<CognitionRequestType>().is_ok() {
        return CognitionTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<MemoryRequestType>().is_ok() {
        return MemoryTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<ExperienceRequestType>().is_ok() {
        return ExperienceTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<ConnectionRequestType>().is_ok() {
        return ConnectionTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<ContinuityRequestType>().is_ok() {
        return ContinuityTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<SearchRequestType>().is_ok() {
        return SearchTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<StorageRequestType>().is_ok() {
        return StorageTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<PressureRequestType>().is_ok() {
        return PressureTools.dispatch(&context, tool_name, params).await;
    }
    // Vocabulary domains
    if tool_name.parse::<LevelRequestType>().is_ok() {
        return LevelTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<TextureRequestType>().is_ok() {
        return TextureTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<SensationRequestType>().is_ok() {
        return SensationTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<NatureRequestType>().is_ok() {
        return NatureTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<PersonaRequestType>().is_ok() {
        return PersonaTools.dispatch(&context, tool_name, params).await;
    }
    if tool_name.parse::<UrgeRequestType>().is_ok() {
        return UrgeTools.dispatch(&context, tool_name, params).await;
    }

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

// ── Auth ───────────────────────────────────────────────────────

/// Resolve a brain-specific config from a Bearer token.
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

// ── Server handler ─────────────────────────────────────────────

/// MCP server wrapping the engine state.
///
/// Each MCP session gets its own `EngineToolBox`. Session-scoped state
/// includes authentication (Bearer token → brain) and the active
/// toolset (which cognitive mode the agent is in).
#[derive(Clone)]
pub struct EngineToolBox {
    state: ServerState,
    session_config: OnceLock<Config>,
    active_toolset: std::sync::Arc<Mutex<Option<Toolset>>>,
}

impl EngineToolBox {
    pub fn new(state: ServerState) -> Self {
        Self {
            state,
            session_config: OnceLock::new(),
            active_toolset: std::sync::Arc::new(Mutex::new(None)),
        }
    }

    fn config(&self) -> &Config {
        self.session_config.get().unwrap_or(self.state.config())
    }

    fn active_toolset(&self) -> Option<Toolset> {
        *self.active_toolset.lock().unwrap()
    }

    fn set_active_toolset(&self, toolset: Option<Toolset>) -> Option<Toolset> {
        let mut guard = self.active_toolset.lock().unwrap();
        let previous = *guard;
        *guard = toolset;
        previous
    }

    fn handle_activate_toolset(&self, params: &str) -> Result<CallToolResult, ToolError> {
        let parsed: ActivateToolsetRequest = serde_json::from_str(params)?;
        let toolset: Toolset = parsed
            .name
            .parse()
            .map_err(|e: String| ToolError::Parameter(e))?;

        self.set_active_toolset(Some(toolset));

        let tools = toolset.tool_names();
        let body = format!(
            "Activated **{}** toolset — {}.\n\n{} tools loaded: {}",
            toolset,
            toolset.description(),
            tools.len(),
            tools.join(", "),
        );

        Ok(CallToolResult::success(vec![Content::text(body)]))
    }

    fn handle_deactivate_toolset(&self) -> CallToolResult {
        let previous = self.set_active_toolset(None);

        let body = match previous {
            Some(toolset) => format!("Deactivated **{}** toolset. Root tools only.", toolset),
            None => "No toolset was active.".to_string(),
        };

        CallToolResult::success(vec![Content::text(body)])
    }

    fn to_rmcp_tool(def: ToolDef) -> Result<RmcpTool, ErrorData> {
        let mut tool = RmcpTool::default();
        tool.name = def.name.to_string().into();
        tool.description = Some(def.description.to_string().into());
        tool.input_schema = serde_json::from_value(def.input_schema).map_err(|err| {
            ErrorData::internal_error(format!("invalid schema for {}: {err}", def.name), None)
        })?;
        Ok(tool)
    }
}

impl ServerHandler for EngineToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_tool_list_changed()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::new(
            "oneiros-engine",
            env!("CARGO_PKG_VERSION"),
        ))
    }

    async fn initialize(
        &self,
        request: rmcp::model::InitializeRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::InitializeResult, ErrorData> {
        if let Some(parts) = context.extensions.get::<axum::http::request::Parts>()
            && let Some(auth) = parts.headers.get("authorization")
            && let Some(token_str) = auth.to_str().ok().and_then(|s| s.strip_prefix("Bearer "))
            && let Some(config) = resolve_config_from_token(&self.state, token_str).await
        {
            let _ = self.session_config.set(config);
        }

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
        let tools: Vec<RmcpTool> = McpServerService::scoped_tools(self.active_toolset())
            .into_iter()
            .map(Self::to_rmcp_tool)
            .collect::<Result<_, ErrorData>>()?;

        Ok(rmcp::model::ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_name = request.name.as_ref();
        let params = serde_json::to_string(&request.arguments.unwrap_or_default())
            .unwrap_or_else(|_| "{}".to_string());

        // Handle root tools directly
        match tool_name {
            "activate-toolset" => {
                let result = self
                    .handle_activate_toolset(&params)
                    .unwrap_or_else(|e| CallToolResult::error(vec![Content::text(e.to_string())]));
                let _ = context.peer.notify_tool_list_changed().await;
                return Ok(result);
            }
            "deactivate-toolset" => {
                let result = self.handle_deactivate_toolset();
                let _ = context.peer.notify_tool_list_changed().await;
                return Ok(result);
            }
            _ => {}
        }

        // Dispatch to domain handlers
        let result = dispatch(&self.state, self.config(), tool_name, &params).await;

        match result {
            Ok(response) => Ok(CallToolResult::success(vec![Content::text(
                response.into_text(),
            )])),
            Err(e) => {
                let body = mcp_error_response(tool_name, &e).into_text();
                Ok(CallToolResult::error(vec![Content::text(body)]))
            }
        }
    }

    // ── Resources ──────────────────────────────────────────────

    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListResourcesResult, ErrorData> {
        Ok(rmcp::model::ListResourcesResult {
            resources: McpServerService::all_resources(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn list_resource_templates(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListResourceTemplatesResult, ErrorData> {
        Ok(rmcp::model::ListResourceTemplatesResult {
            resource_templates: McpServerService::all_resource_templates(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ReadResourceResult, ErrorData> {
        match read_resource(&self.state, self.config(), &request.uri).await {
            Ok(markdown) => Ok(rmcp::model::ReadResourceResult::new(vec![
                ResourceContents::text(markdown, &request.uri).with_mime_type("text/markdown"),
            ])),
            Err(e) => Err(ErrorData::invalid_params(e.to_string(), None)),
        }
    }

    // ── Prompts ────────────────────────────────────────────────

    async fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListPromptsResult, ErrorData> {
        Ok(rmcp::model::ListPromptsResult {
            prompts: McpServerService::prompt_catalog(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::GetPromptResult, ErrorData> {
        match request.name.as_ref() {
            "orient" => {
                let agent_name = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("agent"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("governor.process");

                let text = format!(
                    "# Orientation — {agent_name}\n\n\
                     Check pressure, recent activity, and decide what to do next.\n\n\
                     Use `oneiros-mcp://agent/{agent_name}/pressure` to see cognitive pressure.\n\
                     Use `oneiros-mcp://agent/{agent_name}/cognitions` to see recent thoughts.\n"
                );

                let mut result = rmcp::model::GetPromptResult::new(vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    text,
                )]);
                result.description = Some("Orientation for an agent".into());
                Ok(result)
            }
            "toolsets" => {
                let text = McpServerService::render_toolsets_prompt();

                let mut result = rmcp::model::GetPromptResult::new(vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    text,
                )]);
                result.description = Some("Available toolsets and their capabilities".into());
                Ok(result)
            }
            other => Err(ErrorData::invalid_params(
                format!("unknown prompt: {other}"),
                None,
            )),
        }
    }
}
