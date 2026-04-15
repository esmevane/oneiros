use crate::*;

pub struct LevelView {
    response: LevelResponse,
}

impl LevelView {
    pub fn new(response: LevelResponse) -> Self {
        Self { response }
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
