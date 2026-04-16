use crate::*;

pub struct SearchPresenter {
    response: SearchResponse,
}

impl SearchPresenter {
    pub fn new(response: SearchResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            SearchResponse::Results(results) => {
                let mut md = format!(
                    "# Search: {}\n\n{} results\n\n",
                    results.query,
                    results.results.len()
                );
                for result in &results.results {
                    md.push_str(&format!(
                        "- **{}** ({}): {}\n",
                        result.kind, result.resource_ref, result.content
                    ));
                }
                McpResponse::new(md)
            }
        }
    }

    pub fn render(self) -> Rendered<SearchResponse> {
        let prompt = self.render_prompt();
        let text = self.render_text();

        Rendered::new(self.response, prompt, text)
    }

    fn render_prompt(&self) -> String {
        match &self.response {
            SearchResponse::Results(results) => {
                if results.results.is_empty() {
                    return format!("No results for '{}'.", results.query);
                }

                let mut out = format!("Search results for '{}':\n\n", results.query);
                for result in &results.results {
                    let content = result.content.as_str();
                    let truncated = if content.len() > 80 {
                        let end = content.floor_char_boundary(80);
                        format!("{}...", &content[..end])
                    } else {
                        content.to_string()
                    };
                    out.push_str(&format!(
                        "  [{}] {}\n    {}\n\n",
                        result.kind,
                        truncated,
                        RefToken::new(result.resource_ref.clone())
                    ));
                }
                out
            }
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            SearchResponse::Results(results) => {
                if results.results.is_empty() {
                    format!("No results for '{}'.", results.query)
                } else {
                    format!(
                        "{} result{} for '{}'.",
                        results.results.len(),
                        if results.results.len() == 1 { "" } else { "s" },
                        results.query
                    )
                }
            }
        }
    }
}
