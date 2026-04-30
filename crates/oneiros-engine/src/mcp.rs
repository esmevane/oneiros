//! MCP server — tools, resources, and prompts for agent consumption.
//!
//! Tools are write operations (8 total). Resources are read operations
//! rendered as markdown. Prompts assemble identity context.
//!
//! Session authentication: if the MCP client sends an `Authorization:
//! Bearer <token>` header during the initialize handshake, the session
//! resolves to the brain associated with that token. Without a token,
//! the session uses the server's default brain.

use std::sync::OnceLock;

use rmcp::model::{
    CallToolResult, Content, GetPromptResult, Implementation, ListPromptsResult,
    ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, PaginatedRequestParams,
    ReadResourceResult, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::{ErrorData, ServerHandler};

use crate::*;

/// Errors that can occur during MCP tool dispatch or resource reading.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    /// An application-level error from the engine.
    #[error("Application error: {0}")]
    App(#[from] Error),

    /// The input could not be deserialized.
    #[error("Malformed input: {0}")]
    Malformed(#[from] serde_json::Error),

    /// The requested tool name is not handled by any domain.
    #[error("Unknown tool: {0}")]
    UnknownTool(String),

    /// A parameter could not be deserialized or was otherwise invalid.
    #[error("Parameter error: {0}")]
    Parameter(String),

    /// The requested entity was not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// A write operation was requested through the resource (read) path.
    #[error("Not a resource: {0}")]
    NotAResource(String),
}

fn all_tools() -> Vec<ToolDef> {
    [
        AgentMcp.defs(),
        CognitionMcp.defs(),
        MemoryMcp.defs(),
        ExperienceMcp.defs(),
        ConnectionMcp.defs(),
        SearchMcp.defs(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[tracing::instrument(skip_all, fields(tool = %tool_name), err(Display))]
async fn dispatch(
    state: &ServerState,
    config: &Config,
    tool_name: &ToolName,
    params: &serde_json::Value,
) -> Result<McpResponse, ToolError> {
    let context = state
        .project_log(config.clone())
        .map_err(|e| ToolError::App(e.into()))?;

    let name = tool_name.as_str();

    if name.parse::<AgentRequestType>().is_ok() {
        return AgentMcp.dispatch(&context, tool_name, params).await;
    }
    if name.parse::<CognitionRequestType>().is_ok() {
        return CognitionMcp.dispatch(&context, tool_name, params).await;
    }
    if name.parse::<MemoryRequestType>().is_ok() {
        return MemoryMcp.dispatch(&context, tool_name, params).await;
    }
    if name.parse::<ExperienceRequestType>().is_ok() {
        return ExperienceMcp.dispatch(&context, tool_name, params).await;
    }
    if name.parse::<ConnectionRequestType>().is_ok() {
        return ConnectionMcp.dispatch(&context, tool_name, params).await;
    }
    if name.parse::<SearchRequestType>().is_ok() {
        return SearchMcp.dispatch(&context, tool_name, params).await;
    }

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

fn mcp_error_response(error: &ToolError) -> McpResponse {
    let body = error.to_string();
    let hints = match error {
        ToolError::UnknownTool(_) | ToolError::NotAResource(_) => vec![
            Hint::inspect(ResourcePath::Agents.uri(), "See available agents"),
            Hint::suggest("search-query", "Search across everything"),
        ],
        ToolError::App(_) | ToolError::NotFound(_) => vec![
            Hint::inspect(ResourcePath::Agents.uri(), "See available agents"),
            Hint::suggest("search-query", "Search across everything"),
        ],
        ToolError::Parameter(_) | ToolError::Malformed(_) => vec![Hint::inspect(
            ResourcePath::Agents.uri(),
            "See available agents",
        )],
    };
    McpResponse::new(body).hints(hints)
}

fn all_resources() -> Vec<ResourceDef> {
    [
        AgentMcp.resources(),
        LevelMcp.resources(),
        TextureMcp.resources(),
        SensationMcp.resources(),
        NatureMcp.resources(),
        PersonaMcp.resources(),
        UrgeMcp.resources(),
        ContinuityMcp.resources(),
        PressureMcp.resources(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn all_resource_templates() -> Vec<ResourceTemplateDef> {
    [
        AgentMcp.resource_templates(),
        CognitionMcp.resource_templates(),
        MemoryMcp.resource_templates(),
        ExperienceMcp.resource_templates(),
        ConnectionMcp.resource_templates(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

async fn read_resource(context: &ProjectLog, uri: &ResourceUri) -> Result<McpResponse, ToolError> {
    let path = uri.path();

    let Some(request) = path.as_request() else {
        return AgentMcp.read_resource_special(context, path).await;
    };

    match request {
        ResourceRequest::Agent(req) => AgentMcp.resource(context, &req).await,
        ResourceRequest::Cognition(req) => CognitionMcp.resource(context, &req).await,
        ResourceRequest::Memory(req) => MemoryMcp.resource(context, &req).await,
        ResourceRequest::Experience(req) => ExperienceMcp.resource(context, &req).await,
        ResourceRequest::Connection(req) => ConnectionMcp.resource(context, &req).await,
        ResourceRequest::Level(req) => LevelMcp.resource(context, &req).await,
        ResourceRequest::Texture(req) => TextureMcp.resource(context, &req).await,
        ResourceRequest::Sensation(req) => SensationMcp.resource(context, &req).await,
        ResourceRequest::Nature(req) => NatureMcp.resource(context, &req).await,
        ResourceRequest::Persona(req) => PersonaMcp.resource(context, &req).await,
        ResourceRequest::Urge(req) => UrgeMcp.resource(context, &req).await,
        ResourceRequest::Pressure(req) => PressureMcp.resource(context, &req).await,
        ResourceRequest::Continuity(req) => ContinuityMcp.resource(context, &req).await,
    }
}

async fn resolve_config_from_token(state: &ServerState, token_str: &str) -> Option<Config> {
    let token = Token::from(token_str).decode().ok()?;

    let scope = ComposeScope::new(state.config().clone()).host().ok()?;
    let ticket = TicketRepo::new(&scope)
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

#[derive(Clone)]
pub struct EngineToolBox {
    state: ServerState,
    session_config: OnceLock<Config>,
}

impl EngineToolBox {
    pub fn new(state: ServerState) -> Self {
        Self {
            state,
            session_config: OnceLock::new(),
        }
    }

    fn config(&self) -> &Config {
        self.session_config.get().unwrap_or(self.state.config())
    }
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
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tools: Vec<Tool> = all_tools()
            .into_iter()
            .map(|t| {
                let mut tool = Tool::default();
                tool.name = t.name.to_string().into();
                tool.description = Some(t.description.to_string().into());
                tool.input_schema = serde_json::from_value(t.input_schema).map_err(|err| {
                    ErrorData::internal_error(format!("invalid schema for {}: {err}", t.name), None)
                })?;
                Ok(tool)
            })
            .collect::<Result<_, ErrorData>>()?;

        Ok(ListToolsResult {
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
        let tool_name = ToolName::new(request.name.as_ref());
        let params = serde_json::to_value(request.arguments.unwrap_or_default())
            .unwrap_or_else(|_| serde_json::json!({}));

        match dispatch(&self.state, self.config(), &tool_name, &params).await {
            Ok(response) => Ok(CallToolResult::success(vec![Content::text(
                response.into_text(),
            )])),
            Err(e) => {
                let response = mcp_error_response(&e);
                Ok(CallToolResult::error(vec![Content::text(
                    response.into_text(),
                )]))
            }
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        let mut resources = Vec::new();
        for r in all_resources() {
            let raw = rmcp::model::RawResource {
                uri: r.uri,
                name: r.name,
                title: None,
                description: Some(r.description.to_string()),
                mime_type: Some(r.mime_type),
                size: None,
                icons: None,
                meta: None,
            };
            resources.push(rmcp::model::Annotated::new(raw, None));
        }

        Ok(ListResourcesResult {
            resources,
            next_cursor: None,
            meta: None,
        })
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
        let mut resource_templates = Vec::new();
        for r in all_resource_templates() {
            let raw = rmcp::model::RawResourceTemplate {
                uri_template: r.uri_template,
                name: r.name,
                title: None,
                description: Some(r.description.to_string()),
                mime_type: Some(r.mime_type),
                icons: None,
            };
            resource_templates.push(rmcp::model::Annotated::new(raw, None));
        }

        Ok(ListResourceTemplatesResult {
            resource_templates,
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        let context = self
            .state
            .project_log(self.config().clone())
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        let uri: ResourceUri = request
            .uri
            .parse()
            .map_err(|e: ResourceUriError| ErrorData::invalid_params(e.to_string(), None))?;

        match read_resource(&context, &uri).await {
            Ok(response) => Ok(ReadResourceResult::new(vec![
                rmcp::model::ResourceContents::TextResourceContents {
                    uri: uri.to_string(),
                    mime_type: Some("text/markdown".to_string()),
                    text: response.into_text(),
                    meta: None,
                },
            ])),
            Err(e) => Err(ErrorData::internal_error(e.to_string(), None)),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        let prompts = vec![rmcp::model::Prompt::new(
            "dream",
            Some("Restore an agent's full identity and cognitive context"),
            Some(vec![
                rmcp::model::PromptArgument::new("agent")
                    .with_description("The agent name to dream")
                    .with_required(true),
            ]),
        )];

        Ok(ListPromptsResult {
            prompts,
            next_cursor: None,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        match request.name.as_str() {
            "dream" => {
                let agent_name = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("agent"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ErrorData::invalid_params("Missing required argument: agent", None)
                    })?;

                let context = self
                    .state
                    .project_log(self.config().clone())
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

                let request: DreamAgent = DreamAgent::builder_v1()
                    .agent(AgentName::new(agent_name))
                    .build()
                    .into();

                let response =
                    ContinuityService::dream(&context, &request, &DreamOverrides::default())
                        .await
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

                match response {
                    ContinuityResponse::Dreaming(DreamingResponse::V1(details)) => {
                        let dream = details.context;
                        let text = DreamTemplate::new(&dream).to_string();
                        Ok(
                            GetPromptResult::new(vec![rmcp::model::PromptMessage::new_text(
                                rmcp::model::PromptMessageRole::User,
                                text,
                            )])
                            .with_description(format!("Dream context for {agent_name}")),
                        )
                    }
                    _ => Err(ErrorData::internal_error(
                        "Unexpected response from dream",
                        None,
                    )),
                }
            }
            _ => Err(ErrorData::invalid_params(
                format!("Unknown prompt: {}", request.name),
                None,
            )),
        }
    }
}
