use crate::*;

pub struct PersonaView {
    response: PersonaResponse,
}

impl PersonaView {
    pub fn new(response: PersonaResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            PersonaResponse::Personas(listed) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|w| (w.data.name.to_string(), w.data.description.to_string()))
                    .collect();
                Self::vocabulary_table("Personas", &items)
            }
            PersonaResponse::PersonaDetails(wrapped) => {
                let items = vec![(
                    wrapped.data.name.to_string(),
                    wrapped.data.description.to_string(),
                )];
                Self::vocabulary_table("Persona", &items)
            }
            PersonaResponse::NoPersonas => Self::vocabulary_table("Personas", &[]),
            PersonaResponse::PersonaSet(name) => McpResponse::new(format!("Persona set: {name}")),
            PersonaResponse::PersonaRemoved(name) => {
                McpResponse::new(format!("Persona removed: {name}"))
            }
        }
    }

    fn vocabulary_table(title: &str, items: &[(String, String)]) -> McpResponse {
        let mut md = format!("# {title}\n\n");
        if items.is_empty() {
            md.push_str(&format!("No {title} configured.\n"));
        } else {
            md.push_str("| Name | Description |\n");
            md.push_str("|------|-------------|\n");
            for (name, desc) in items {
                md.push_str(&format!("| {name} | {desc} |\n"));
            }
        }
        McpResponse::new(md).hint(Hint::inspect(ResourcePath::Agents.uri(), "View all agents"))
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
