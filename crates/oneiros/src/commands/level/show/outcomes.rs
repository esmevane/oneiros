use oneiros_model::LevelRecord;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowLevelOutcomes {
    #[outcome(message("Level '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    LevelDetails(LevelRecord),
}
