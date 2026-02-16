use oneiros_model::Cognition;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListCognitionsOutcomes {
    #[outcome(message("No cognitions found."))]
    NoCognitions,

    #[outcome(
        message("Cognitions: {0:?}"),
        prompt(
            "Which of these are still working threads? Consolidate what's crystallized with `oneiros memory add <agent>`."
        )
    )]
    Cognitions(Vec<Cognition>),
}
