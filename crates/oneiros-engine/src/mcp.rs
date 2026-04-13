//! MCP server — toolset-aware adapter with resources and prompts.
//!
//! The server exposes a **root** layer (dashboard resources, toolset management
//! tools, orientation prompts) that is always available. The agent can activate
//! a **Toolset** to load additional tools for a specific cognitive moment.
//!
//! Primitives used:
//! - **Tools**: root tools (activate/deactivate, pressure, status) + active toolset's tools
//! - **Resources**: read-only windows into cognitive state (dream, pressure, status)
//! - **Prompts**: interaction templates that guide toolset selection
//!
//! Session authentication: if the MCP client sends an `Authorization:
//! Bearer <token>` header during the initialize handshake, the session
//! resolves to the brain associated with that token. Without a token,
//! the session uses the server's default brain.

use std::sync::OnceLock;

use rmcp::model::{
    Annotated, CallToolResult, Content, GetPromptRequestParams, GetPromptResult,
    Implementation, ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult,
    PaginatedRequestParams, PromptArgument, PromptMessage, PromptMessageRole,
    ReadResourceRequestParams, ReadResourceResult, ResourceContents, ServerCapabilities, ServerInfo,
    Tool,
};
use rmcp::{ErrorData, ServerHandler};
use tokio::sync::RwLock;

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

// ---------------------------------------------------------------------------
// Root tools — always available regardless of active toolset
// ---------------------------------------------------------------------------

/// Tools that are always present: toolset management + dashboard essentials.
fn root_tools() -> Vec<ToolDef> {
    [
        vec![
            crate::values::Tool::<ActivateToolset>::new(
                "activate-toolset",
                "Load a toolset — changes which tools are available. \
                 Options: lifecycle, capture, garden, admin, distribute",
            )
            .def(),
            crate::values::Tool::<DeactivateToolset>::new(
                "deactivate-toolset",
                "Unload the active toolset, returning to root-only tools",
            )
            .def(),
        ],
        PressureTools.defs(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

// ---------------------------------------------------------------------------
// Resources — read-only windows, always available
// ---------------------------------------------------------------------------

/// Resource templates — parameterized read-only data windows.
fn resource_templates() -> Vec<Annotated<rmcp::model::RawResourceTemplate>> {
    use rmcp::model::{AnnotateAble, RawResourceTemplate};

    vec![
        RawResourceTemplate::new("oneiros://agent/{name}/dream", "Dream context")
            .with_description("Full assembled identity and cognitive context for an agent")
            .with_mime_type("text/markdown")
            .no_annotation(),
        RawResourceTemplate::new("oneiros://agent/{name}/pressure", "Pressure gauge")
            .with_description("Current cognitive pressure readings and urge levels")
            .with_mime_type("text/markdown")
            .no_annotation(),
        RawResourceTemplate::new("oneiros://agent/{name}/guidebook", "Guidebook")
            .with_description("How to use cognitive tools — progressive reference")
            .with_mime_type("text/markdown")
            .no_annotation(),
    ]
}

/// Read a resource by URI.
async fn read_resource(
    state: &ServerState,
    config: &Config,
    uri: &str,
) -> Result<Vec<ResourceContents>, ToolError> {
    // Parse oneiros://agent/{name}/{kind}
    let path = uri
        .strip_prefix("oneiros://agent/")
        .ok_or_else(|| ToolError::Parameter(format!("Unknown resource URI: {uri}")))?;

    let (agent_name, kind) = path
        .split_once('/')
        .ok_or_else(|| ToolError::Parameter(format!("Malformed resource URI: {uri}")))?;

    let context = state
        .project_context(config.clone())
        .map_err(|e| ToolError::Domain(e.to_string()))?;

    match kind {
        "dream" => {
            let request = DreamAgent::builder()
                .agent(AgentName::new(agent_name))
                .build();
            let overrides = DreamOverrides::default();
            let value = ContinuityService::dream(&context, &request, &overrides)
                .await
                .map_err(Error::from)?;
            let text = serde_json::to_value(&value)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_else(|| serde_json::to_string_pretty(&value).unwrap_or_default());
            Ok(vec![ResourceContents::text(text, uri)])
        }
        "pressure" => {
            let request = GetPressure::builder()
                .agent(AgentName::new(agent_name))
                .build();
            let value = PressureService::get(&context, &request)
                .await
                .map_err(Error::from)?;
            let text = serde_json::to_string_pretty(&value)
                .unwrap_or_else(|_| format!("{value:?}"));
            Ok(vec![ResourceContents::text(text, uri)])
        }
        "guidebook" => {
            let request = GuidebookAgent::builder()
                .agent(AgentName::new(agent_name))
                .build();
            let overrides = DreamOverrides::default();
            let value = ContinuityService::guidebook(&context, &request, &overrides)
                .map_err(Error::from)?;
            let text = serde_json::to_value(&value)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_else(|| serde_json::to_string_pretty(&value).unwrap_or_default());
            Ok(vec![ResourceContents::text(text, uri)])
        }
        other => Err(ToolError::Parameter(format!(
            "Unknown resource kind: '{other}'. Available: dream, pressure, guidebook"
        ))),
    }
}

// ---------------------------------------------------------------------------
// Prompts — interaction templates that guide toolset selection
// ---------------------------------------------------------------------------

/// Available MCP prompts.
fn prompt_catalog() -> Vec<rmcp::model::Prompt> {
    vec![
        rmcp::model::Prompt::new(
            "orient",
            Some("Read cognitive state and get toolset suggestions"),
            Some(vec![
                PromptArgument::new("agent")
                    .with_description("Agent name (e.g., governor.process)")
                    .with_required(true),
            ]),
        ),
        rmcp::model::Prompt::new(
            "toolsets",
            Some("List all available toolsets and their descriptions"),
            None,
        ),
    ]
}

/// Handle a prompt request.
async fn handle_prompt(
    state: &ServerState,
    config: &Config,
    name: &str,
    arguments: &Option<serde_json::Map<String, serde_json::Value>>,
) -> Result<GetPromptResult, ToolError> {
    match name {
        "orient" => {
            let agent_name = arguments
                .as_ref()
                .and_then(|a| a.get("agent"))
                .and_then(|v| v.as_str())
                .ok_or_else(|| ToolError::Parameter("'agent' argument is required".into()))?;

            let context = state
                .project_context(config.clone())
                .map_err(|e| ToolError::Domain(e.to_string()))?;

            let request = GetPressure::builder()
                .agent(AgentName::new(agent_name))
                .build();
            let pressure = PressureService::get(&context, &request)
                .await
                .map_err(Error::from)?;

            let pressure_text = serde_json::to_string_pretty(&pressure)
                .unwrap_or_else(|_| format!("{pressure:?}"));

            let mut lines = vec![
                format!("## {agent_name} — Orientation\n"),
                format!("### Pressure\n\n{pressure_text}\n"),
                "### Available Toolsets\n".to_string(),
            ];

            for toolset in Toolset::all() {
                lines.push(format!(
                    "- **{}** ({} tools) — {}",
                    toolset,
                    toolset.tool_count(),
                    toolset.description(),
                ));
            }

            lines.push("\nUse `activate-toolset` to load the tools for your current moment.".into());

            let content = lines.join("\n");
            Ok(GetPromptResult::new(vec![PromptMessage::new_text(
                PromptMessageRole::Assistant,
                content,
            )]))
        }
        "toolsets" => {
            let mut lines = vec!["## Available Toolsets\n".to_string()];
            for toolset in Toolset::all() {
                lines.push(format!(
                    "### {} ({} tools)\n\n{}\n\nTools: {}\n",
                    toolset,
                    toolset.tool_count(),
                    toolset.description(),
                    toolset
                        .defs()
                        .iter()
                        .map(|t| t.name.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                ));
            }
            Ok(GetPromptResult::new(vec![PromptMessage::new_text(
                PromptMessageRole::Assistant,
                lines.join("\n"),
            )]))
        }
        other => Err(ToolError::UnknownTool(format!("Unknown prompt: {other}"))),
    }
}

// ---------------------------------------------------------------------------
// Domain dispatch — routes tool names to domain dispatchers
// ---------------------------------------------------------------------------

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
    if tool_name.parse::<BookmarkRequestType>().is_ok() {
        return BookmarkTools.dispatch(state, tool_name, params).await;
    }

    // All other tools work through ProjectContext
    let context = state
        .project_context(config.clone())
        .map_err(|e| ToolError::Domain(e.to_string()))?;

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

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

// ---------------------------------------------------------------------------
// Token resolution
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// EngineToolBox — per-session MCP handler
// ---------------------------------------------------------------------------

/// MCP server wrapping the server state.
///
/// Each MCP session gets its own `EngineToolBox`. During the initialize
/// handshake, if a Bearer token is present, the session resolves to
/// the brain associated with that token. Otherwise it uses the server's
/// default brain config.
///
/// The active toolset controls which tools appear in `list_tools`.
/// Activating a toolset notifies the client to re-fetch the tool list.
#[derive(Clone)]
pub struct EngineToolBox {
    state: ServerState,
    /// Session-resolved config, set during initialize from Bearer token.
    /// Falls back to state.config() if not set.
    session_config: OnceLock<Config>,
    /// The currently active toolset for this session.
    /// None means root-only (dashboard + toolset management).
    active_toolset: std::sync::Arc<RwLock<Option<Toolset>>>,
}

impl EngineToolBox {
    pub fn new(state: ServerState) -> Self {
        Self {
            state,
            session_config: OnceLock::new(),
            active_toolset: std::sync::Arc::new(RwLock::new(None)),
        }
    }

    /// The config for this session — resolved from token if authenticated,
    /// otherwise the server default.
    fn config(&self) -> &Config {
        self.session_config.get().unwrap_or(self.state.config())
    }

    /// Collect tool definitions for the current session state.
    async fn current_tools(&self) -> Vec<ToolDef> {
        let mut tools = root_tools();

        if let Some(toolset) = self.active_toolset.read().await.as_ref() {
            tools.extend(toolset.defs());
        }

        tools
    }
}

/// Convert a ToolDef into an rmcp Tool.
fn to_rmcp_tool(t: ToolDef) -> Tool {
    let mut tool = Tool::default();
    tool.name = t.name.to_string().into();
    tool.description = Some(t.description.to_string().into());
    tool.input_schema =
        serde_json::from_value(t.input_schema).expect("schema should be a JSON object");
    tool
}

impl ServerHandler for EngineToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::new("oneiros-engine", env!("CARGO_PKG_VERSION")))
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
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, ErrorData> {
        let tools = self.current_tools().await.into_iter().map(to_rmcp_tool).collect();

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

        // Handle toolset management tools
        match tool_name {
            "activate-toolset" => {
                let req: ActivateToolset = serde_json::from_str(&params)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

                let toolset: Toolset = req.toolset.parse()
                    .map_err(|e: String| ErrorData::invalid_params(e, None))?;

                let description = toolset.description().to_string();
                let count = toolset.tool_count();
                let name = toolset.to_string();

                *self.active_toolset.write().await = Some(toolset);

                // Notify the client to re-fetch the tool list
                let _ = context.peer.notify_tool_list_changed().await;

                return Ok(CallToolResult::success(vec![Content::text(format!(
                    "Loaded {name} toolset ({count} tools). {description}"
                ))]));
            }
            "deactivate-toolset" => {
                let had_toolset = self.active_toolset.read().await.is_some();
                *self.active_toolset.write().await = None;

                if had_toolset {
                    let _ = context.peer.notify_tool_list_changed().await;
                }

                return Ok(CallToolResult::success(vec![Content::text(
                    "Toolset deactivated. Root tools only.",
                )]));
            }
            _ => {}
        }

        // Dispatch to domain handlers
        match dispatch(&self.state, self.config(), tool_name, &params).await {
            Ok(value) => Ok(CallToolResult::success(vec![
                Content::json(value).expect("content"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    // -----------------------------------------------------------------------
    // Resources
    // -----------------------------------------------------------------------

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
        Ok(ListResourceTemplatesResult {
            resource_templates: resource_templates(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        // No concrete (non-template) resources — all are parameterized
        Ok(ListResourcesResult {
            resources: vec![],
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        let contents = read_resource(&self.state, self.config(), &request.uri)
            .await
            .map_err(|e| ErrorData::resource_not_found(e.to_string(), None))?;

        Ok(ReadResourceResult::new(contents))
    }

    // -----------------------------------------------------------------------
    // Prompts
    // -----------------------------------------------------------------------

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        Ok(ListPromptsResult {
            prompts: prompt_catalog(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        handle_prompt(&self.state, self.config(), &request.name, &request.arguments)
            .await
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))
    }
}
