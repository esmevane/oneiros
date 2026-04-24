use crate::*;

pub struct SearchPresenter {
    response: SearchResponse,
}

impl SearchPresenter {
    pub fn new(response: SearchResponse) -> Self {
        Self { response }
    }

    /// MCP rendering — kind-grouped sections, one per content type. Agents
    /// orient by kind first, content second. Facets render as a final
    /// "palace map" section so the surrounding shape is visible without
    /// an extra round trip.
    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            SearchResponse::Results(ResultsResponse::V1(results)) => {
                let mut md = format!(
                    "# Search: {}\n\n{} of {} results\n",
                    results.query,
                    results.hits.len(),
                    results.total,
                );

                for (heading, group) in group_by_kind(&results.hits) {
                    if group.is_empty() {
                        continue;
                    }
                    md.push_str(&format!("\n## {heading}\n\n"));
                    for hit in &group {
                        md.push_str(&render_hit_item(hit));
                    }
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

    /// CLI rendering — flat table with a `Kind` column. FTS5 relevance
    /// order across kinds is preserved row-by-row, since rank means more
    /// than kind grouping for terminal scanning.
    fn render_prompt(&self) -> String {
        match &self.response {
            SearchResponse::Results(ResultsResponse::V1(results)) => {
                let heading = if results.query.as_str().is_empty() {
                    "Browse results".to_string()
                } else {
                    format!("Search results for '{}'", results.query)
                };
                if results.hits.is_empty() {
                    return format!("{heading}: none.");
                }

                let mut table = Table::new(vec![
                    Column::key("kind", "Kind"),
                    Column::key("content", "Content").max(60),
                    Column::key("ref_token", "Ref"),
                ]);

                for hit in &results.hits {
                    let owned = hit.content();
                    table.push_row(vec![
                        hit.kind().to_string(),
                        owned.to_string(),
                        RefToken::new(hit.resource_ref()).to_string(),
                    ]);
                }

                let raw = format!("{} of {} total", results.hits.len(), results.total);
                let summary = raw.muted();
                format!("{heading}\n\n{summary}\n\n{table}")
            }
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            SearchResponse::Results(ResultsResponse::V1(results)) => {
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

/// Bucket hits by kind, preserving FTS5 rank within each bucket. Returns
/// stable section ordering: cognitions, memories, experiences, agents.
fn group_by_kind(hits: &[Hit]) -> [(&'static str, Vec<&Hit>); 4] {
    let mut cognitions = Vec::new();
    let mut memories = Vec::new();
    let mut experiences = Vec::new();
    let mut agents = Vec::new();
    for hit in hits {
        match hit {
            Hit::Cognition(_) => cognitions.push(hit),
            Hit::Memory(_) => memories.push(hit),
            Hit::Experience(_) => experiences.push(hit),
            Hit::Agent(_) => agents.push(hit),
        }
    }
    [
        ("Cognitions", cognitions),
        ("Memories", memories),
        ("Experiences", experiences),
        ("Agents", agents),
    ]
}

/// Render one hit as a kind-shaped MCP item. Each kind shows the facet
/// that lists already lead with — texture for cognitions, level for
/// memories, sensation for experiences, persona for agents.
fn render_hit_item(hit: &Hit) -> String {
    let ref_token = RefToken::new(hit.resource_ref());
    match hit {
        Hit::Cognition(c) => format!(
            "- **{}** — {}\n  {}\n  {}\n",
            c.texture, c.created_at, c.content, ref_token
        ),
        Hit::Memory(m) => format!(
            "- **{}** — {}\n  {}\n  {}\n",
            m.level, m.created_at, m.content, ref_token
        ),
        Hit::Experience(e) => format!(
            "- **{}** — {}\n  {}\n  {}\n",
            e.sensation, e.created_at, e.description, ref_token
        ),
        Hit::Agent(a) => format!(
            "- **{}** ({})\n  {}\n  {}\n",
            a.name, a.persona, a.description, ref_token
        ),
    }
}
