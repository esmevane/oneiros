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
            LevelResponse::Levels(LevelsResponse::V1(listed)) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|item| (item.name.to_string(), item.description.to_string()))
                    .collect();
                Self::vocabulary_table("Levels", &items)
            }
            LevelResponse::LevelDetails(LevelDetailsResponse::V1(details)) => {
                let items = vec![(
                    details.level.name.to_string(),
                    details.level.description.to_string(),
                )];
                Self::vocabulary_table("Level", &items)
            }
            LevelResponse::NoLevels => Self::vocabulary_table("Levels", &[]),
            LevelResponse::LevelSet(LevelSetResponse::V1(set)) => {
                McpResponse::new(format!("Level set: {}", set.level.name))
            }
            LevelResponse::LevelRemoved(LevelRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Level removed: {}", removed.name))
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

    pub fn render(self) -> Rendered<LevelResponse> {
        match self.response {
            LevelResponse::LevelSet(LevelSetResponse::V1(set)) => {
                let prompt =
                    Confirmation::new("Level", set.level.name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("level".to_string()).build(),
                );
                Rendered::new(
                    LevelResponse::LevelSet(LevelSetResponse::V1(set)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            LevelResponse::LevelDetails(LevelDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.level.name.to_string())
                    .field("description:", details.level.description.to_string())
                    .field("prompt:", details.level.prompt.to_string())
                    .to_string();
                Rendered::new(
                    LevelResponse::LevelDetails(LevelDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            LevelResponse::Levels(LevelsResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("description", "Description").max(60),
                ]);
                for item in &listed.items {
                    table.push_row(vec![item.name.to_string(), item.description.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    LevelResponse::Levels(LevelsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            LevelResponse::NoLevels => Rendered::new(
                LevelResponse::NoLevels,
                format!("{}", "No levels configured.".muted()),
                String::new(),
            ),
            LevelResponse::LevelRemoved(LevelRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Level", removed.name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("level".to_string()).build(),
                );
                Rendered::new(
                    LevelResponse::LevelRemoved(LevelRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
