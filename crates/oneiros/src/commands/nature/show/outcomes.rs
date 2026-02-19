use oneiros_model::Nature;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowNatureOutcomes {
    #[outcome(message("Nature details: {0:?}"))]
    NatureDetails(Nature),
}
