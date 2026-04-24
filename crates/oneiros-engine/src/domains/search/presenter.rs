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
                    "# Search: {}\n\n{} of {} results\n\n",
                    results.query,
                    results.hits.len(),
                    results.total,
                );
                for hit in &results.hits {
                    md.push_str(&format!(
                        "- **{}** ({}): {}\n",
                        hit.kind, hit.resource_ref, hit.content
                    ));
                }
                if !results.facets.is_empty() {
                    md.push_str("\n## Facets\n\n");
                    for group in &results.facets.0 {
                        md.push_str(&format!("- **{:?}**: ", group.facet));
                        let parts: Vec<String> = group
                            .buckets
                            .iter()
                            .map(|b| format!("{} ({})", b.value, b.count))
                            .collect();
                        md.push_str(&parts.join(", "));
                        md.push('\n');
                    }
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
                let heading = if results.query.as_str().is_empty() {
                    "Browse results".to_string()
                } else {
                    format!("Search results for '{}'", results.query)
                };
                if results.hits.is_empty() {
                    return format!("{heading}: none.");
                }

                let mut out = format!("{heading}:\n\n");
                for hit in &results.hits {
                    let content = hit.content.as_str();
                    let truncated = if content.len() > 80 {
                        let end = content.floor_char_boundary(80);
                        format!("{}...", &content[..end])
                    } else {
                        content.to_string()
                    };
                    out.push_str(&format!(
                        "  [{}] {}\n    {}\n\n",
                        hit.kind,
                        truncated,
                        RefToken::new(hit.resource_ref.clone())
                    ));
                }
                out
            }
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            SearchResponse::Results(results) => {
                if results.hits.is_empty() {
                    format!("No results for '{}'.", results.query)
                } else {
                    format!(
                        "{} result{} for '{}'.",
                        results.hits.len(),
                        if results.hits.len() == 1 { "" } else { "s" },
                        results.query
                    )
                }
            }
        }
    }
}
