use oneiros_model::CognitionId;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum AddCognitionOutcomes {
    #[outcome(message("Cognition added: {0}"))]
    CognitionAdded(CognitionId),
}
