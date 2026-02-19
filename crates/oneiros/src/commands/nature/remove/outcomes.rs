use oneiros_model::NatureName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveNatureOutcomes {
    #[outcome(message("Nature '{0}' removed."))]
    NatureRemoved(NatureName),
}
