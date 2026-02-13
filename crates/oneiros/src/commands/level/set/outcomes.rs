use oneiros_model::LevelName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetLevelOutcomes {
    #[outcome(message("Level '{0}' set."))]
    LevelSet(LevelName),
}
