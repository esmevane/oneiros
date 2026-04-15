//! Memory view — presentation authority for the memory domain.
//!
//! Owns the response and produces `Rendered<MemoryResponse>` with
//! navigational hints.

use crate::*;

pub struct MemoryView<'a> {
    response: MemoryResponse,
    command: &'a MemoryCommands,
}

impl<'a> MemoryView<'a> {
    pub fn new(response: MemoryResponse, command: &'a MemoryCommands) -> Self {
        Self { response, command }
    }

    pub fn render(self) -> Rendered<MemoryResponse> {
        match (self.response, self.command) {
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
                let prompt = Detail::new(wrapped.data.level.to_string())
                    .field("content:", wrapped.data.content.to_string())
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
            (MemoryResponse::Memories(listed), MemoryCommands::List(listing)) => {
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
                        wrapped.data.level.to_string(),
                        wrapped.data.content.to_string(),
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
