use oneiros_model::CognitionId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
pub struct CognitionAddedResult {
    pub id: CognitionId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddCognitionOutcomes {
    #[outcome(message("Cognition added: {}", .0.id), prompt("{}", .0.gauge))]
    CognitionAdded(CognitionAddedResult),
}
