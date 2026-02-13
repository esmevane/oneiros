use oneiros_model::MemoryId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddMemoryOutcomes {
    #[outcome(message("Memory added: {0}"))]
    MemoryAdded(MemoryId),
}
