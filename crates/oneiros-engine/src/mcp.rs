//! MCP tool server — thin rmcp adapter delegating to per-domain dispatchers.
//!
//! Each domain owns its tool catalog in `features/mcp.rs`. This module
//! collects them into an rmcp ServerHandler, routing tool calls to the
//! appropriate domain dispatcher.

use rmcp::model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo, Tool};
use rmcp::{ErrorData, ServerHandler};
use serde_json::json;

use crate::*;

/// Errors that can occur during MCP tool dispatch.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
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
        level_mcp::tool_defs(),
        texture_mcp::tool_defs(),
        sensation_mcp::tool_defs(),
        nature_mcp::tool_defs(),
        persona_mcp::tool_defs(),
        urge_mcp::tool_defs(),
        agent_mcp::tool_defs(),
        cognition_mcp::tool_defs(),
        memory_mcp::tool_defs(),
        experience_mcp::tool_defs(),
        connection_mcp::tool_defs(),
        lifecycle_mcp::tool_defs(),
        search_mcp::tool_defs(),
        storage_mcp::tool_defs(),
        pressure_mcp::tool_defs(),
    ];
    sources.iter().flat_map(|s| s.iter()).collect()
}

type Dispatcher = fn(&ProjectContext, &str, &str) -> Result<serde_json::Value, ToolError>;

/// Domain dispatch table — routes tool names to domain dispatchers.
fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    // Check each domain's tool catalog
    let dispatchers: &[(&[&str], Dispatcher)] = &[
        (level_mcp::tool_names(), level_mcp::dispatch),
        (texture_mcp::tool_names(), texture_mcp::dispatch),
        (sensation_mcp::tool_names(), sensation_mcp::dispatch),
        (nature_mcp::tool_names(), nature_mcp::dispatch),
        (persona_mcp::tool_names(), persona_mcp::dispatch),
        (urge_mcp::tool_names(), urge_mcp::dispatch),
        (agent_mcp::tool_names(), agent_mcp::dispatch),
        (cognition_mcp::tool_names(), cognition_mcp::dispatch),
        (memory_mcp::tool_names(), memory_mcp::dispatch),
        (experience_mcp::tool_names(), experience_mcp::dispatch),
        (connection_mcp::tool_names(), connection_mcp::dispatch),
        (lifecycle_mcp::tool_names(), lifecycle_mcp::dispatch),
        (search_mcp::tool_names(), search_mcp::dispatch),
        (storage_mcp::tool_names(), storage_mcp::dispatch),
        (pressure_mcp::tool_names(), pressure_mcp::dispatch),
    ];

    for (names, handler) in dispatchers {
        if names.contains(&tool_name) {
            return handler(ctx, tool_name, params);
        }
    }

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

/// MCP tool server wrapping the project context.
#[derive(Clone)]
pub struct EngineToolBox {
    ctx: ProjectContext,
}

impl EngineToolBox {
    pub fn new(ctx: ProjectContext) -> Self {
        Self { ctx }
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
                tool.input_schema = serde_json::from_value(json!({
                    "type": "object",
                    "properties": {},
                }))
                .unwrap();
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

        match dispatch(&self.ctx, tool_name, &params) {
            Ok(value) => Ok(CallToolResult::success(vec![
                Content::json(value).expect("content"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}
