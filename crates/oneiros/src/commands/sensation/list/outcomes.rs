use oneiros_model::Sensation;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListSensationsOutcomes {
    #[outcome(message("No sensations configured."))]
    NoSensations,

    #[outcome(message("Sensations: {0:?}"))]
    Sensations(Vec<Sensation>),
}
