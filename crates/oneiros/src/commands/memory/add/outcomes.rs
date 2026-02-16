use oneiros_model::MemoryId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
pub struct MemoryAddedResult {
    pub id: MemoryId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddMemoryOutcomes {
    #[outcome(message("Memory added: {}", .0.id), prompt("{}", .0.gauge))]
    MemoryAdded(MemoryAddedResult),
}
