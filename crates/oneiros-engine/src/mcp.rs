//! MCP tool server — thin rmcp adapter delegating to per-domain dispatchers.
//!
//! Each domain owns its tool catalog in `features/mcp.rs`. This module
//! collects them into an rmcp ServerHandler, routing tool calls to the
//! appropriate domain dispatcher.

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
async fn dispatch(
    context: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    // System domains
    if ActorTools.names().contains(&tool_name) {
        return ActorTools.dispatch(context, tool_name, params).await;
    }
    if TenantTools.names().contains(&tool_name) {
        return TenantTools.dispatch(context, tool_name, params).await;
    }
    if BrainTools.names().contains(&tool_name) {
        return BrainTools.dispatch(context, tool_name, params).await;
    }
    if TicketTools.names().contains(&tool_name) {
        return TicketTools.dispatch(context, tool_name, params).await;
    }
    // Project domains
    if LevelTools.names().contains(&tool_name) {
        return LevelTools.dispatch(context, tool_name, params).await;
    }
    if TextureTools.names().contains(&tool_name) {
        return TextureTools.dispatch(context, tool_name, params).await;
    }
    if SensationTools.names().contains(&tool_name) {
        return SensationTools.dispatch(context, tool_name, params).await;
    }
    if NatureTools.names().contains(&tool_name) {
        return NatureTools.dispatch(context, tool_name, params).await;
    }
    if PersonaTools.names().contains(&tool_name) {
        return PersonaTools.dispatch(context, tool_name, params).await;
    }
    if UrgeTools.names().contains(&tool_name) {
        return UrgeTools.dispatch(context, tool_name, params).await;
    }
    if AgentTools.names().contains(&tool_name) {
        return AgentTools.dispatch(context, tool_name, params).await;
    }
    if CognitionTools.names().contains(&tool_name) {
        return CognitionTools.dispatch(context, tool_name, params).await;
    }
    if MemoryTools.names().contains(&tool_name) {
        return MemoryTools.dispatch(context, tool_name, params).await;
    }
    if ExperienceTools.names().contains(&tool_name) {
        return ExperienceTools.dispatch(context, tool_name, params).await;
    }
    if ConnectionTools.names().contains(&tool_name) {
        return ConnectionTools.dispatch(context, tool_name, params).await;
    }
    if ContinuityTools.names().contains(&tool_name) {
        return ContinuityTools.dispatch(context, tool_name, params).await;
    }
    if SearchTools.names().contains(&tool_name) {
        return SearchTools.dispatch(context, tool_name, params).await;
    }
    if StorageTools.names().contains(&tool_name) {
        return StorageTools.dispatch(context, tool_name, params).await;
    }
    if PressureTools.names().contains(&tool_name) {
        return PressureTools.dispatch(context, tool_name, params).await;
    }

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

/// MCP tool server wrapping the project context.
#[derive(Clone)]
pub struct EngineToolBox {
    context: ProjectContext,
}

impl EngineToolBox {
    pub fn new(context: ProjectContext) -> Self {
        Self { context }
    }
}

impl ServerHandler for EngineToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new("oneiros-engine", env!("CARGO_PKG_VERSION")),
        )
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

        match dispatch(&self.context, tool_name, &params).await {
            Ok(value) => Ok(CallToolResult::success(vec![
                Content::json(value).expect("content"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}
