use oneiros_model::{Cognition, CognitionId, Identity};
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct CognitionList(pub Vec<Identity<CognitionId, Cognition>>);

impl core::fmt::Display for CognitionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|cognition| format!("{cognition}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListCognitionsOutcomes {
    #[outcome(message("No cognitions found."))]
    NoCognitions,

    #[outcome(
        message("{0}"),
        prompt(
            "Which of these are still working threads? Consolidate what's crystallized with `oneiros memory add <agent>`."
        )
    )]
    Cognitions(CognitionList),
}
