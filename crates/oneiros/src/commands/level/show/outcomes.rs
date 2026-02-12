use oneiros_model::Level;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ShowLevelOutcomes {
    #[outcome(message("Level '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    LevelDetails(Level),
}
