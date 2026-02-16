use oneiros_model::Experience;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowExperienceOutcomes {
    #[outcome(message("Experience details: {0:?}"))]
    ExperienceDetails(Experience),
}
