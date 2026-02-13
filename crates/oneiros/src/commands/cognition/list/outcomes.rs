use oneiros_model::Cognition;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListCognitionsOutcomes {
    #[outcome(message("No cognitions found."))]
    NoCognitions,

    #[outcome(message("Cognitions: {0:?}"))]
    Cognitions(Vec<Cognition>),
}
