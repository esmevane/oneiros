use crate::*;

pub struct LevelView {
    response: LevelResponse,
}

impl LevelView {
    pub fn new(response: LevelResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            LevelResponse::Levels(listed) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|w| (w.data.name.to_string(), w.data.description.to_string()))
                    .collect();
                Self::vocabulary_table("Levels", &items)
            }
            LevelResponse::LevelDetails(wrapped) => {
                let items = vec![(
                    wrapped.data.name.to_string(),
                    wrapped.data.description.to_string(),
                )];
                Self::vocabulary_table("Level", &items)
            }
            LevelResponse::NoLevels => Self::vocabulary_table("Levels", &[]),
            LevelResponse::LevelSet(name) => McpResponse::new(format!("Level set: {name}")),
            LevelResponse::LevelRemoved(name) => McpResponse::new(format!("Level removed: {name}")),
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

    pub fn render(self) -> Rendered<LevelResponse> {
        match self.response {
            LevelResponse::LevelSet(name) => {
                let prompt = Confirmation::new("Level", name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("level".to_string()).build(),
                );
                Rendered::new(LevelResponse::LevelSet(name), prompt, String::new())
                    .with_hints(hints)
            }
            LevelResponse::LevelDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .field("prompt:", wrapped.data.prompt.to_string())
                    .to_string();
                Rendered::new(LevelResponse::LevelDetails(wrapped), prompt, String::new())
            }
            LevelResponse::Levels(listed) => {
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
                Rendered::new(LevelResponse::Levels(listed), prompt, String::new())
            }
            LevelResponse::NoLevels => Rendered::new(
                LevelResponse::NoLevels,
                format!("{}", "No levels configured.".muted()),
                String::new(),
            ),
            LevelResponse::LevelRemoved(name) => {
                let prompt = Confirmation::new("Level", name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("level".to_string()).build(),
                );
                Rendered::new(LevelResponse::LevelRemoved(name), prompt, String::new())
                    .with_hints(hints)
            }
        }
    }
}
