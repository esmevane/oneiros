use crate::*;

pub struct SensationView {
    response: SensationResponse,
}

impl SensationView {
    pub fn new(response: SensationResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<SensationResponse> {
        match self.response {
            SensationResponse::SensationSet(name) => {
                let prompt = Confirmation::new("Sensation", name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("sensation".to_string())
                        .build(),
                );
                Rendered::new(SensationResponse::SensationSet(name), prompt, String::new())
                    .with_hints(hints)
            }
            SensationResponse::SensationDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .field("prompt:", wrapped.data.prompt.to_string())
                    .to_string();
                Rendered::new(
                    SensationResponse::SensationDetails(wrapped),
                    prompt,
                    String::new(),
                )
            }
            SensationResponse::Sensations(listed) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("description", "Description").max(60),
                ]);
                for wrapped in &listed.items {
                    table.push_row(vec![
                        wrapped.data.name.to_string(),
                        wrapped.data.description.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(SensationResponse::Sensations(listed), prompt, String::new())
            }
            SensationResponse::NoSensations => Rendered::new(
                SensationResponse::NoSensations,
                format!("{}", "No sensations configured.".muted()),
                String::new(),
            ),
            SensationResponse::SensationRemoved(name) => {
                let prompt =
                    Confirmation::new("Sensation", name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("sensation".to_string())
                        .build(),
                );
                Rendered::new(
                    SensationResponse::SensationRemoved(name),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
