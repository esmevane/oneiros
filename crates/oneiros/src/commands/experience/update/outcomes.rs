use oneiros_model::ExperienceId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
pub struct ExperienceUpdatedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UpdateExperienceOutcomes {
    #[outcome(message("Experience updated: {}", .0.id), prompt("{}", .0.gauge))]
    ExperienceUpdated(ExperienceUpdatedResult),
}
