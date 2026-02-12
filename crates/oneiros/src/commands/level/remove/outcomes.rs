use oneiros_model::LevelName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum RemoveLevelOutcomes {
    #[outcome(message("Level '{0}' removed."))]
    LevelRemoved(LevelName),
}
