use oneiros_model::NatureName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetNatureOutcomes {
    #[outcome(message("Nature '{0}' set."))]
    NatureSet(NatureName),
}
