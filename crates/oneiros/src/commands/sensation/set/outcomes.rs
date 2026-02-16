use oneiros_model::SensationName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetSensationOutcomes {
    #[outcome(message("Sensation '{0}' set."))]
    SensationSet(SensationName),
}
