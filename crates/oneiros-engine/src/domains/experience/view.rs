//! Experience view — presentation authority for the experience domain.

use crate::*;

pub struct ExperienceView {
    response: ExperienceResponse,
}

impl ExperienceView {
    pub fn new(response: ExperienceResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<ExperienceResponse> {
        match self.response {
            ExperienceResponse::ExperienceCreated(wrapped) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Experience recorded: {}",
                            "✓".success(),
                            ref_token.muted()
                        )
                    })
                    .unwrap_or_default();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(
                    ExperienceResponse::ExperienceCreated(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
            ExperienceResponse::ExperienceDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.sensation.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .to_string();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(
                    ExperienceResponse::ExperienceDetails(wrapped),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            ExperienceResponse::Experiences(listed) => {
                let mut table = Table::new(vec![
                    Column::key("sensation", "Sensation"),
                    Column::key("description", "Description").max(60),
                    Column::key("ref_token", "Ref"),
                ]);
                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|t| t.to_string())
                        .unwrap_or_default();
                    table.push_row(vec![
                        wrapped.data.sensation.to_string(),
                        wrapped.data.description.to_string(),
                        ref_token,
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(
                    ExperienceResponse::Experiences(listed),
                    prompt,
                    String::new(),
                )
            }
            ExperienceResponse::NoExperiences => Rendered::new(
                ExperienceResponse::NoExperiences,
                format!("{}", "No experiences.".muted()),
                String::new(),
            ),
            ExperienceResponse::ExperienceUpdated(wrapped) => {
                let subject = wrapped
                    .meta()
                    .ref_token()
                    .map(|ref_token| {
                        format!(
                            "{} Experience updated: {}",
                            "✓".success(),
                            ref_token.muted()
                        )
                    })
                    .unwrap_or_default();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(
                    ExperienceResponse::ExperienceUpdated(wrapped),
                    subject,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
