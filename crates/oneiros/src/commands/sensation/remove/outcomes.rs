use oneiros_model::SensationName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveSensationOutcomes {
    #[outcome(message("Sensation '{0}' removed."))]
    SensationRemoved(SensationName),
}
