//! Memory view — presentation authority for the memory domain.
//!
//! Owns the response and produces `Rendered<MemoryResponse>` with
//! navigational hints.

use crate::*;

pub struct MemoryView<'a> {
    response: MemoryResponse,
    request: &'a MemoryRequest,
}

impl<'a> MemoryView<'a> {
    pub fn new(response: MemoryResponse, request: &'a MemoryRequest) -> Self {
        Self { response, request }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            MemoryResponse::MemoryAdded(wrapped) => {
                let ref_token = wrapped.data.ref_token();
                McpResponse::new(format!(
                    "Memory consolidated ({}).\n\n**level:** {}\n**ref:** {}",
                    wrapped.data.level(),
                    wrapped.data.level(),
                    ref_token
                ))
                .hint_set(HintSet::mutation(
                    MutationHints::builder().ref_token(ref_token).build(),
                ))
            }
            MemoryResponse::MemoryDetails(wrapped) => {
                let ref_token = wrapped.data.ref_token();
                McpResponse::new(format!(
                    "# Memory\n\n**level:** {}\n**agent:** {}\n**created:** {}\n\n{}\n",
                    wrapped.data.level(),
                    wrapped.data.agent_id(),
                    wrapped.data.created_at(),
                    wrapped.data.content()
                ))
                .hint(Hint::suggest(
                    format!("create-connection <nature> {ref_token} <target>"),
                    "Connect to something related",
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            MemoryResponse::Memories(listed) => {
                let title = match self.request {
                    MemoryRequest::ListMemories(listing) => match &listing.agent {
                        Some(agent) => format!("# Memories — {agent}\n\n"),
                        None => "# Memories\n\n".to_string(),
                    },
                    _ => "# Memories\n\n".to_string(),
                };
                let mut md = format!("{title}{} of {} total\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    md.push_str(&format!(
                        "### {} — {}\n{}\n\n",
                        wrapped.data.level(),
                        wrapped.data.created_at(),
                        wrapped.data.content()
                    ));
                }
                let mut response = McpResponse::new(md)
                    .hint(Hint::suggest("add-memory", "Consolidate what matters"));
                if let MemoryRequest::ListMemories(listing) = self.request
                    && let Some(agent) = &listing.agent
                {
                    response = response.hint(Hint::inspect(
                        ResourcePath::AgentExperiences(agent.clone()).uri(),
                        "Browse experiences",
                    ));
                }
                response
            }
            MemoryResponse::NoMemories => McpResponse::new("No memories yet.")
                .hint(Hint::suggest("add-memory", "Consolidate what matters")),
        }
    }

    pub fn render(self) -> Rendered<MemoryResponse> {
        match (self.response, self.request) {
            (MemoryResponse::MemoryAdded(wrapped), _) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!("{} Memory recorded: {}", "✓".success(), ref_token.muted())
                    })
                    .unwrap_or_default();

                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };

                Rendered::new(MemoryResponse::MemoryAdded(wrapped), subject, String::new())
                    .with_hints(hints)
            }
            (MemoryResponse::MemoryDetails(wrapped), _) => {
                let prompt = Detail::new(wrapped.data.level().to_string())
                    .field("content:", wrapped.data.content().to_string())
                    .to_string();

                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };

                Rendered::new(
                    MemoryResponse::MemoryDetails(wrapped),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            (MemoryResponse::Memories(listed), MemoryRequest::ListMemories(listing)) => {
                let mut table = Table::new(vec![
                    Column::key("level", "Level"),
                    Column::key("content", "Content").max(60),
                    Column::key("ref_token", "Ref"),
                ]);

                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|t| t.to_string())
                        .unwrap_or_default();
                    table.push_row(vec![
                        wrapped.data.level().to_string(),
                        wrapped.data.content().to_string(),
                        ref_token,
                    ]);
                }

                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );

                let hints = match &listing.agent {
                    Some(agent) => HintSet::listing(
                        ListingHints::builder()
                            .agent(agent.clone())
                            .has_more(listed.len() < listed.total)
                            .build(),
                    ),
                    None => HintSet::None,
                };

                Rendered::new(MemoryResponse::Memories(listed), prompt, String::new())
                    .with_hints(hints)
            }
            (MemoryResponse::NoMemories, _) => Rendered::new(
                MemoryResponse::NoMemories,
                format!("{}", "No memories.".muted()),
                String::new(),
            ),
            // unreachable: Memories only comes from List
            (response, _) => Rendered::new(response, String::new(), String::new()),
        }
    }
}
