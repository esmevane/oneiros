use oneiros_model::ExperienceId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
pub struct RefAddedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RefAddOutcomes {
    #[outcome(message("Reference added to experience: {}", .0.id), prompt("{}", .0.gauge))]
    RefAdded(RefAddedResult),
}
