//! Layer collectors — thin assemblers that merge domain registrations.
//!
//! Each layer collector imports the participating domains and wires them
//! together. Domains that don't participate simply don't appear.

use axum::Router;

use crate::domains;
use crate::ports::AppContext;

/// Collect all HTTP routes into one router.
pub fn http_router(ctx: AppContext) -> Router {
    Router::new()
        .nest("/agents", domains::agent::http::routes())
        // other domains would be .nest()'d here
        .with_state(ctx)
}

/// Dispatch an MCP tool call to the right domain.
pub fn dispatch_tool(
    ctx: &AppContext,
    tool_name: &str,
    params: &str,
) -> Result<domains::agent::mcp::ToolResult, domains::agent::mcp::ToolError> {
    // Check agent tools
    if domains::agent::mcp::tool_names().contains(&tool_name) {
        return domains::agent::mcp::dispatch(ctx, tool_name, params);
    }

    // other domains would be checked here

    Err(domains::agent::mcp::ToolError::UnknownTool(tool_name.to_string()))
}
