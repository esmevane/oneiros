use crate::*;

pub struct NatureView {
    response: NatureResponse,
}

impl NatureView {
    pub fn new(response: NatureResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<NatureResponse> {
        match self.response {
            NatureResponse::NatureSet(name) => {
                let prompt = Confirmation::new("Nature", name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("nature".to_string())
                        .build(),
                );
                Rendered::new(NatureResponse::NatureSet(name), prompt, String::new())
                    .with_hints(hints)
            }
            NatureResponse::NatureDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .field("prompt:", wrapped.data.prompt.to_string())
                    .to_string();
                Rendered::new(
                    NatureResponse::NatureDetails(wrapped),
                    prompt,
                    String::new(),
                )
            }
            NatureResponse::Natures(listed) => {
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
                Rendered::new(NatureResponse::Natures(listed), prompt, String::new())
            }
            NatureResponse::NoNatures => Rendered::new(
                NatureResponse::NoNatures,
                format!("{}", "No natures configured.".muted()),
                String::new(),
            ),
            NatureResponse::NatureRemoved(name) => {
                let prompt = Confirmation::new("Nature", name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("nature".to_string())
                        .build(),
                );
                Rendered::new(NatureResponse::NatureRemoved(name), prompt, String::new())
                    .with_hints(hints)
            }
        }
    }
}
