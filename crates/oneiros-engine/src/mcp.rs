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
        continuity_mcp::tool_defs(),
        search_mcp::tool_defs(),
        storage_mcp::tool_defs(),
        pressure_mcp::tool_defs(),
    ];
    sources.iter().flat_map(|s| s.iter()).collect()
}

/// Domain dispatch table — routes tool names to domain dispatchers.
async fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    if level_mcp::tool_names().contains(&tool_name) {
        return level_mcp::dispatch(ctx, tool_name, params).await;
    }
    if texture_mcp::tool_names().contains(&tool_name) {
        return texture_mcp::dispatch(ctx, tool_name, params).await;
    }
    if sensation_mcp::tool_names().contains(&tool_name) {
        return sensation_mcp::dispatch(ctx, tool_name, params).await;
    }
    if nature_mcp::tool_names().contains(&tool_name) {
        return nature_mcp::dispatch(ctx, tool_name, params).await;
    }
    if persona_mcp::tool_names().contains(&tool_name) {
        return persona_mcp::dispatch(ctx, tool_name, params).await;
    }
    if urge_mcp::tool_names().contains(&tool_name) {
        return urge_mcp::dispatch(ctx, tool_name, params).await;
    }
    if agent_mcp::tool_names().contains(&tool_name) {
        return agent_mcp::dispatch(ctx, tool_name, params).await;
    }
    if cognition_mcp::tool_names().contains(&tool_name) {
        return cognition_mcp::dispatch(ctx, tool_name, params).await;
    }
    if memory_mcp::tool_names().contains(&tool_name) {
        return memory_mcp::dispatch(ctx, tool_name, params).await;
    }
    if experience_mcp::tool_names().contains(&tool_name) {
        return experience_mcp::dispatch(ctx, tool_name, params).await;
    }
    if connection_mcp::tool_names().contains(&tool_name) {
        return connection_mcp::dispatch(ctx, tool_name, params).await;
    }
    if continuity_mcp::tool_names().contains(&tool_name) {
        return continuity_mcp::dispatch(ctx, tool_name, params).await;
    }
    if search_mcp::tool_names().contains(&tool_name) {
        return search_mcp::dispatch(ctx, tool_name, params);
    }
    if storage_mcp::tool_names().contains(&tool_name) {
        return storage_mcp::dispatch(ctx, tool_name, params).await;
    }
    if pressure_mcp::tool_names().contains(&tool_name) {
        return pressure_mcp::dispatch(ctx, tool_name, params);
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

        match dispatch(&self.ctx, tool_name, &params).await {
            Ok(value) => Ok(CallToolResult::success(vec![
                Content::json(value).expect("content"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}
