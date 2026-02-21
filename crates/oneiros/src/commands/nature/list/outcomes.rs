use oneiros_model::NatureRecord;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListNaturesOutcomes {
    #[outcome(message("No natures configured."))]
    NoNatures,

    #[outcome(message("Natures: {0:?}"))]
    Natures(Vec<NatureRecord>),
}
