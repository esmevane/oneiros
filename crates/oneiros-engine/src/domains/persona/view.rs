use crate::*;

pub struct PersonaView {
    response: PersonaResponse,
}

impl PersonaView {
    pub fn new(response: PersonaResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<PersonaResponse> {
        match self.response {
            PersonaResponse::PersonaSet(name) => {
                let prompt = Confirmation::new("Persona", name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("persona".to_string())
                        .build(),
                );
                Rendered::new(PersonaResponse::PersonaSet(name), prompt, String::new())
                    .with_hints(hints)
            }
            PersonaResponse::PersonaDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .field("prompt:", wrapped.data.prompt.to_string())
                    .to_string();
                Rendered::new(
                    PersonaResponse::PersonaDetails(wrapped),
                    prompt,
                    String::new(),
                )
            }
            PersonaResponse::Personas(listed) => {
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
                Rendered::new(PersonaResponse::Personas(listed), prompt, String::new())
            }
            PersonaResponse::NoPersonas => Rendered::new(
                PersonaResponse::NoPersonas,
                format!("{}", "No personas configured.".muted()),
                String::new(),
            ),
            PersonaResponse::PersonaRemoved(name) => {
                let prompt = Confirmation::new("Persona", name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("persona".to_string())
                        .build(),
                );
                Rendered::new(PersonaResponse::PersonaRemoved(name), prompt, String::new())
                    .with_hints(hints)
            }
        }
    }
}
