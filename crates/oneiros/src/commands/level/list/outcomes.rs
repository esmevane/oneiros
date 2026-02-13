use oneiros_model::Level;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListLevelsOutcomes {
    #[outcome(message("No levels configured."))]
    NoLevels,

    #[outcome(message("Levels: {0:?}"))]
    Levels(Vec<Level>),
}
