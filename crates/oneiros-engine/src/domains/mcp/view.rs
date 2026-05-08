//! MCP view — presentation authority for the MCP config domain.
//!
//! Maps MCP config responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub(crate) struct McpView {
    response: McpResponses,
}

impl McpView {
    pub(crate) fn new(response: McpResponses) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<McpResponses> {
        match self.response {
            McpResponses::McpConfigWritten(McpConfigWrittenResponse::V1(details)) => {
                let path = details.path;
                let prompt = format!(
                    "{} MCP config written to {}.",
                    "✓".success(),
                    path.display()
                );
                Rendered::new(
                    McpResponses::McpConfigWritten(
                        McpConfigWrittenResponse::builder_v1()
                            .path(path)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            McpResponses::McpConfigExists(McpConfigExistsResponse::V1(details)) => {
                let path = details.path;
                let prompt = format!(
                    "{}",
                    format!("MCP config already exists at {}. Skipped.", path.display()).muted()
                );
                Rendered::new(
                    McpResponses::McpConfigExists(
                        McpConfigExistsResponse::builder_v1()
                            .path(path)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
