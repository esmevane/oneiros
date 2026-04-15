//! MCP view — presentation authority for the MCP config domain.
//!
//! Maps MCP config responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub struct McpView {
    response: McpConfigResponse,
}

impl McpView {
    pub fn new(response: McpConfigResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<McpConfigResponse> {
        match self.response {
            McpConfigResponse::McpConfigWritten(path) => {
                let prompt = format!(
                    "{} MCP config written to {}.",
                    "✓".success(),
                    path.display()
                );
                Rendered::new(
                    McpConfigResponse::McpConfigWritten(path),
                    prompt,
                    String::new(),
                )
            }
            McpConfigResponse::McpConfigExists(path) => {
                let prompt = format!(
                    "{}",
                    format!("MCP config already exists at {}. Skipped.", path.display()).muted()
                );
                Rendered::new(
                    McpConfigResponse::McpConfigExists(path),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
