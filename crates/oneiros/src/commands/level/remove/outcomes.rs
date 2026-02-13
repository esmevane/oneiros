use oneiros_model::LevelName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveLevelOutcomes {
    #[outcome(message("Level '{0}' removed."))]
    LevelRemoved(LevelName),
}
