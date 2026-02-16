use oneiros_model::ExperienceId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
pub struct ExperienceCreatedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateExperienceOutcomes {
    #[outcome(message("Experience created: {}", .0.id), prompt("{}", .0.gauge))]
    ExperienceCreated(ExperienceCreatedResult),
}
