use oneiros_model::Level;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListLevelsOutcomes {
    #[outcome(message("No levels configured."))]
    NoLevels,

    #[outcome(message("Levels: {0:?}"))]
    Levels(Vec<Level>),
}
