use crate::*;

/// MCP tool facade for the mcp domain itself — the activate/deactivate
/// toolset operations that form the root layer of the MCP surface.
pub struct McpTools;

impl McpTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        McpServerService::root_tool_defs()
    }
}
