use crate::*;

/// An authored MCP response: markdown body + navigational hints.
///
/// Every tool call and resource read produces one of these.
/// `into_text()` renders it for the MCP protocol.
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

    pub fn hint(mut self, hint: Hint) -> Self {
        self.hints.push(hint);
        self
    }

    pub fn hints(mut self, hints: Vec<Hint>) -> Self {
        self.hints.extend(hints);
        self
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_body_only_when_no_hints() {
        let response = McpResponse::new("Hello");
        assert_eq!(response.into_text(), "Hello");
    }

    #[test]
    fn renders_hints_section() {
        let response = McpResponse::new("Done.").hint(Hint::suggest("search-query", "Find things"));
        let text = response.into_text();
        assert!(text.contains("Done."));
        assert!(text.contains("Hints"));
    }
}
