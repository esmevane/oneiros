use oneiros_model::SensationRecord;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowSensationOutcomes {
    #[outcome(message("Sensation details: {0:?}"))]
    SensationDetails(SensationRecord),
}
