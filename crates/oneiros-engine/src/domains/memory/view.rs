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
            MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added)) => {
                let ref_token = RefToken::new(Ref::memory(added.memory.id));
                McpResponse::new(format!(
                    "Memory consolidated ({}).\n\n**level:** {}\n**ref:** {}",
                    added.memory.level, added.memory.level, ref_token
                ))
                .hint_set(HintSet::mutation(
                    MutationHints::builder().ref_token(ref_token).build(),
                ))
            }
            MemoryResponse::MemoryDetails(MemoryDetailsResponse::V1(details)) => {
                let ref_token = RefToken::new(Ref::memory(details.memory.id));
                McpResponse::new(format!(
                    "# Memory\n\n**level:** {}\n**agent:** {}\n**created:** {}\n\n{}\n",
                    details.memory.level,
                    details.memory.agent_id,
                    details.memory.created_at,
                    details.memory.content
                ))
                .hint(Hint::suggest(
                    format!("create-connection <nature> {ref_token} <target>"),
                    "Connect to something related",
                ))
                .hint(Hint::suggest("search-query", "Search for related entities"))
            }
            MemoryResponse::Memories(MemoriesResponse::V1(listed)) => {
                let title = match self.request {
                    MemoryRequest::ListMemories(ListMemories::V1(listing)) => {
                        match &listing.agent {
                            Some(agent) => format!("# Memories — {agent}\n\n"),
                            None => "# Memories\n\n".to_string(),
                        }
                    }
                    _ => "# Memories\n\n".to_string(),
                };
                let mut md = format!(
                    "{title}{} of {} total\n\n",
                    listed.items.len(),
                    listed.total
                );
                for item in &listed.items {
                    md.push_str(&format!(
                        "### {} — {}\n{}\n\n",
                        item.level, item.created_at, item.content
                    ));
                }
                let mut response = McpResponse::new(md)
                    .hint(Hint::suggest("add-memory", "Consolidate what matters"));
                if let MemoryRequest::ListMemories(ListMemories::V1(listing)) = self.request
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
            (MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added)), _) => {
                let ref_token = RefToken::new(Ref::memory(added.memory.id));
                let subject = format!(
                    "{} Memory recorded: {}",
                    "✓".success(),
                    ref_token.clone().muted()
                );
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());

                Rendered::new(
                    MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added)),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            (MemoryResponse::MemoryDetails(MemoryDetailsResponse::V1(details)), _) => {
                let prompt = Detail::new(details.memory.level.to_string())
                    .field("content:", details.memory.content.to_string())
                    .to_string();
                let ref_token = RefToken::new(Ref::memory(details.memory.id));
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());

                Rendered::new(
                    MemoryResponse::MemoryDetails(MemoryDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            (
                MemoryResponse::Memories(MemoriesResponse::V1(listed)),
                MemoryRequest::ListMemories(ListMemories::V1(listing)),
            ) => {
                let mut table = Table::new(vec![
                    Column::key("level", "Level"),
                    Column::key("content", "Content").max(60),
                    Column::key("ref_token", "Ref"),
                ]);

                for item in &listed.items {
                    let ref_token = RefToken::new(Ref::memory(item.id));
                    table.push_row(vec![
                        item.level.to_string(),
                        item.content.to_string(),
                        ref_token.to_string(),
                    ]);
                }

                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );

                let hints = match &listing.agent {
                    Some(agent) => HintSet::listing(
                        ListingHints::builder()
                            .agent(agent.clone())
                            .has_more(listed.items.len() < listed.total)
                            .build(),
                    ),
                    None => HintSet::None,
                };

                Rendered::new(
                    MemoryResponse::Memories(MemoriesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
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
