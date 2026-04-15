//! MCP response — authored markdown with navigational hints.
//!
//! The structured intermediate between domain results and MCP
//! `Content::text()`. Sibling to `Response<T>` — both carry domain
//! data with metadata, but `McpResponse` renders for agent consumption.

use crate::*;

/// An authored MCP response: markdown body + navigational hints.
///
/// Every tool call and resource read produces one of these.
/// `into_content()` renders it for the MCP protocol.
pub struct McpResponse {
    body: String,
    hints: Vec<Hint>,
}

impl McpResponse {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            hints: Vec::new(),
        }
    }

    /// Add a single hint.
    pub fn hint(mut self, hint: Hint) -> Self {
        self.hints.push(hint);
        self
    }

    /// Add multiple hints.
    pub fn hints(mut self, hints: Vec<Hint>) -> Self {
        self.hints.extend(hints);
        self
    }

    /// Add hints from a HintSet.
    pub fn hint_set(self, set: HintSet) -> Self {
        self.hints(set.hints())
    }

    /// Render into MCP text content.
    pub fn into_text(self) -> String {
        let mut text = self.body;
        if !self.hints.is_empty() {
            let section = HintTemplate { hints: &self.hints }.to_string();
            text.push_str(&section);
        }
        text
    }
}

/// Build an `McpResponse` for an error with recovery hints.
pub fn mcp_error_response(tool_name: &str, error: &ToolError) -> McpResponse {
    let body = error.to_string();
    let hints = error_hints(tool_name, error);
    McpResponse { body, hints }
}

/// Generate recovery hints for errors.
fn error_hints(_tool_name: &str, error: &ToolError) -> Vec<Hint> {
    match error {
        ToolError::UnknownTool(_) => vec![
            Hint::inspect("activate-toolset", "Load a toolset to access more tools"),
            Hint::inspect("oneiros-mcp://agents", "See available agents"),
        ],
        ToolError::App(_) | ToolError::Domain(_) => vec![
            Hint::inspect("oneiros-mcp://agents", "See available agents"),
            Hint::suggest("search-query", "Search across everything"),
        ],
        ToolError::Parameter(_) | ToolError::Malformed(_) => vec![Hint::inspect(
            "activate-toolset",
            "Check available tools and their schemas",
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_response_renders_body_only_when_no_hints() {
        let response = McpResponse::new("Hello");
        assert_eq!(response.into_text(), "Hello");
    }

    #[test]
    fn mcp_response_renders_hints_section() {
        let response = McpResponse::new("Done.").hint(Hint::suggest("search-query", "Find things"));
        let text = response.into_text();
        assert!(text.contains("Done."));
        assert!(text.contains("## Hints"));
        assert!(text.contains("**suggest**"));
    }

    #[test]
    fn mcp_error_response_includes_hints() {
        let error = ToolError::UnknownTool("ghost".to_string());
        let response = mcp_error_response("ghost", &error);
        let text = response.into_text();
        assert!(text.contains("ghost"));
        assert!(text.contains("## Hints"));
    }
}
