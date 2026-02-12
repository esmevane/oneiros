use oneiros_model::LevelName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum SetLevelOutcomes {
    #[outcome(message("Level '{0}' set."))]
    LevelSet(LevelName),
}
