//! Cognition view — presentation authority for the cognition domain.
//!
//! Owns the response and produces `Rendered<CognitionResponse>` with
//! navigational hints.

use crate::*;

pub struct CognitionView<'a> {
    response: CognitionResponse,
    command: &'a CognitionCommands,
}

impl<'a> CognitionView<'a> {
    pub fn new(response: CognitionResponse, command: &'a CognitionCommands) -> Self {
        Self { response, command }
    }

    pub fn render(self) -> Rendered<CognitionResponse> {
        match (self.response, self.command) {
            (CognitionResponse::CognitionAdded(wrapped), CognitionCommands::Add(addition)) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Cognition recorded: {}",
                            "✓".success(),
                            ref_token.muted()
                        )
                    })
                    .unwrap_or_default();

                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => HintSet::cognition_added(
                        CognitionAddedHints::builder()
                            .agent(addition.agent.clone())
                            .ref_token(ref_token)
                            .build(),
                    ),
                    None => HintSet::None,
                };

                Rendered::new(
                    CognitionResponse::CognitionAdded(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            (CognitionResponse::CognitionDetails(wrapped), _) => {
                let prompt = Detail::new(wrapped.data.texture.to_string())
                    .field("content:", wrapped.data.content.to_string())
                    .to_string();

                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };

                Rendered::new(
                    CognitionResponse::CognitionDetails(wrapped),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            (CognitionResponse::Cognitions(listed), _) => {
                let mut table = Table::new(vec![
                    Column::key("texture", "Texture"),
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
                        wrapped.data.texture.to_string(),
                        wrapped.data.content.to_string(),
                        ref_token,
                    ]);
                }

                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );

                Rendered::new(CognitionResponse::Cognitions(listed), prompt, String::new())
            }
            (CognitionResponse::NoCognitions, _) => Rendered::new(
                CognitionResponse::NoCognitions,
                format!("{}", "No cognitions.".muted()),
                String::new(),
            ),
            // unreachable: CognitionAdded only comes from Add
            (response, _) => Rendered::new(response, String::new(), String::new()),
        }
    }
}
