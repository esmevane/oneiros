use oneiros_model::Experience;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListExperiencesOutcomes {
    #[outcome(message("No experiences found."))]
    NoExperiences,

    #[outcome(message("Experiences: {0:?}"))]
    Experiences(Vec<Experience>),
}
