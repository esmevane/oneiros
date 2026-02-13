use oneiros_model::CognitionId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddCognitionOutcomes {
    #[outcome(message("Cognition added: {0}"))]
    CognitionAdded(CognitionId),
}
