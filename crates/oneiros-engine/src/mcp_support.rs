//! Shared MCP tool dispatch support types.
//!
//! All domain `features/mcp` modules import `ToolError` from here so the
//! error vocabulary is uniform across every tool dispatcher.

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
