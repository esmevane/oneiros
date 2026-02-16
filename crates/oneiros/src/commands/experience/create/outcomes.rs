use oneiros_model::ExperienceId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateExperienceOutcomes {
    #[outcome(message("Experience created: {0}"))]
    ExperienceCreated(ExperienceId),
}
