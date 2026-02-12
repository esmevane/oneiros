use oneiros_model::Cognition;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListCognitionsOutcomes {
    #[outcome(message("No cognitions found."))]
    NoCognitions,

    #[outcome(message("Cognitions: {0:?}"))]
    Cognitions(Vec<Cognition>),
}
