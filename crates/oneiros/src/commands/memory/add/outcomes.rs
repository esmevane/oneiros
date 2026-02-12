use oneiros_model::MemoryId;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum AddMemoryOutcomes {
    #[outcome(message("Memory added: {0}"))]
    MemoryAdded(MemoryId),
}
