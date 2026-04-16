use crate::*;

pub struct NatureView {
    response: NatureResponse,
}

impl NatureView {
    pub fn new(response: NatureResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            NatureResponse::Natures(listed) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|w| (w.data.name.to_string(), w.data.description.to_string()))
                    .collect();
                Self::vocabulary_table("Natures", &items)
            }
            NatureResponse::NatureDetails(wrapped) => {
                let items = vec![(
                    wrapped.data.name.to_string(),
                    wrapped.data.description.to_string(),
                )];
                Self::vocabulary_table("Nature", &items)
            }
            NatureResponse::NoNatures => Self::vocabulary_table("Natures", &[]),
            NatureResponse::NatureSet(name) => McpResponse::new(format!("Nature set: {name}")),
            NatureResponse::NatureRemoved(name) => {
                McpResponse::new(format!("Nature removed: {name}"))
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
